use std::{collections::HashMap, str::FromStr, time::Duration};

use anyhow::{bail, Error, Result};
use async_std::{
    fs::read_to_string,
    net::{SocketAddr, UdpSocket},
    path::Path,
    task::sleep,
};
use log::{debug, info, trace};
use mlua::prelude::*;
use phorcys_osc::prelude::*;
use time::{format_description::parse as parse_time_format, OffsetDateTime};

#[derive(Debug)]
pub struct DateTimeSender<'lua> {
    script_name: String,
    lua_base: &'lua Lua,
    elements: HashMap<String, ConversionElement<'lua>>,
    // functions: LuaTable,
}

#[derive(Debug)]
pub struct ConversionElement<'lua> {
    address: String,
    value_type: Option<TargetType>,
    value_function: LuaFunction<'lua>,
}

/// Target type of value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TargetType {
    /// The value will be sent as integer.
    Int,

    /// The value will be sent as float.
    Float,
}

impl FromStr for TargetType {
    type Err = Error;

    fn from_str(s: &str) -> Result<TargetType, Error> {
        match s {
            "int" => Ok(TargetType::Int),
            "float" => Ok(TargetType::Float),
            _ => bail!("Invalid target type: {}", s),
        }
    }
}

impl<'lua> DateTimeSender<'lua> {
    /// Constructs `DateTimeSender`.
    pub async fn new(lua: &'lua Lua, filename: impl AsRef<str>) -> Result<DateTimeSender<'lua>> {
        let filename = filename.as_ref();
        let source = read_to_string(filename).await?;

        let script_table: LuaValue = lua.load(&source).call(())?;
        let table = match script_table {
            LuaValue::Table(t) => t,
            _ => bail!(
                "{:?} returned invalid value; a script should return table",
                filename
            ),
        };

        let mut elements = HashMap::new();
        for pair in table.pairs::<LuaValue, LuaValue>() {
            let (key, value) = pair?;
            let key_string = match key {
                LuaValue::String(s) => s.to_str()?.to_string(),
                LuaValue::Integer(i) => i.to_string(),
                LuaValue::Number(f) => f.to_string(),
                _ => bail!("Invalid key type: {:?}", key),
            };
            let info_table = match value {
                LuaValue::Table(t) => t,
                _ => bail!("Convertion element must have a table value"),
            };

            let address = info_table.get("address")?;
            let value_type = info_table
                .get::<_, Option<String>>("value_type")?
                .map(|s| s.parse())
                .transpose()?;
            let value_function = info_table.get("value_function")?;
            elements.insert(
                key_string,
                ConversionElement {
                    address,
                    value_type,
                    value_function,
                },
            );
        }

        Ok(DateTimeSender {
            script_name: filename.to_string(),
            lua_base: lua,
            elements,
        })
    }

    /// Executes DateTime conversion via Lua script, and gets OSC messages.
    pub fn convert(&self, datetime: OffsetDateTime) -> Result<Vec<OscMessage>> {
        debug!("Converting DateTime for {}", self.script_name);

        let mut messages = Vec::with_capacity(self.elements.len());
        for (key, element) in &self.elements {
            trace!("-> {}", key);
            let dt_table = self.create_datetime_table(datetime)?;
            let converted_value: LuaValue = element.value_function.call(dt_table)?;
            let osc_value = DateTimeSender::to_osc_value(&converted_value, element.value_type)?;
            let message = OscMessageBuilder::new(&element.address)?
                .push_argument(osc_value)
                .build();
            messages.push(message);
        }

        Ok(messages)
    }

    /// Creates a DateTime table for conversion element argument.
    fn create_datetime_table(&self, datetime: OffsetDateTime) -> Result<LuaTable> {
        let table = self.lua_base.create_table()?;
        table.set("hour", datetime.hour())?;
        table.set("minute", datetime.minute())?;
        table.set("second", datetime.second())?;
        table.set("month", datetime.month() as u8)?;
        table.set("day", datetime.day())?;
        Ok(table)
    }

    /// Converts `LuaValue` to `OscValue` with type hinting.
    fn to_osc_value(lua_value: &LuaValue, type_hint: Option<TargetType>) -> Result<OscValue> {
        let value = match (lua_value, type_hint) {
            (LuaValue::Integer(v), Some(TargetType::Int) | None) => OscValue::Int32(*v as i32),
            (LuaValue::Integer(v), Some(TargetType::Float)) => OscValue::Float32(*v as f32),
            (LuaValue::Number(v), Some(TargetType::Float) | None) => OscValue::Float32(*v as f32),
            (LuaValue::Number(v), Some(TargetType::Int)) => OscValue::Int32(*v as i32),
            _ => bail!("Invalid value conversion"),
        };
        Ok(value)
    }
}

/*
/// Runs main thread.
pub async fn run(senders: &[DateTimePartSender], interval: u64, address: SocketAddr) -> Result<()> {
    let log_format = parse_time_format("[hour]:[minute]:[second]")?;

    info!("Using {} as sending socket address", address);
    let send_socket = UdpSocket::bind({
        let mut bind_addr = address;
        bind_addr.set_port(0);
        bind_addr
    })
    .await?;
    send_socket.connect(address).await?;

    info!("Started to send packets...");
    loop {
        let now = OffsetDateTime::now_local()?;
        debug!("Sending {}", now.format(&log_format)?);

        for sender in senders {
            let message = sender.construct_packet(now)?.serialize();
            send_socket.send(&message).await?;
        }
        sleep(Duration::from_secs(interval)).await;
    }
}
*/
