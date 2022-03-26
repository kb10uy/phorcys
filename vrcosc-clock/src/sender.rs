use std::{str::FromStr, time::Duration};

use anyhow::{bail, Error, Result};
use async_std::{
    fs::read_to_string,
    net::{SocketAddr, UdpSocket},
    task::sleep,
};
use log::info;
use mlua::prelude::*;
use phorcys_osc::prelude::*;
use time::OffsetDateTime;

/// DateTime struct for Lua interop.
/// Behaves as UserData object in Lua.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LuaDateTime {
    hour: u8,
    minute: u8,
    second: u8,
    month: u8,
    day: u8,
}

impl LuaUserData for LuaDateTime {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("hour", |_, this| Ok(this.hour));
        fields.add_field_method_get("minute", |_, this| Ok(this.minute));
        fields.add_field_method_get("second", |_, this| Ok(this.second));
        fields.add_field_method_get("month", |_, this| Ok(this.month));
        fields.add_field_method_get("day", |_, this| Ok(this.day));
    }
}

impl From<OffsetDateTime> for LuaDateTime {
    fn from(dt: OffsetDateTime) -> LuaDateTime {
        LuaDateTime {
            hour: dt.hour(),
            minute: dt.minute(),
            second: dt.second(),
            month: dt.month() as u8,
            day: dt.day(),
        }
    }
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

/// Represents an element as a pair of address and function.
#[derive(Debug)]
pub struct ConversionElement {
    /// OSC address to send at.
    address: String,

    /// Type hint for number conversion.
    value_type: Option<TargetType>,

    /// Lua function to convert DateTime into OSC value.
    value_function_name: String,
}

/// Provides construction information for Sender.
pub struct SenderConstructor {
    /// Script filename to load.
    pub script_filename: String,

    /// OSC port of VRChat.
    pub osc_socket_address: SocketAddr,

    /// Sending interval.
    pub interval: Duration,
}

pub async fn run_script(constructor: SenderConstructor) -> Result<()> {
    let (lua, elements) = convert_table(&constructor.script_filename).await?;
    info!(
        "Constructed {} element(s) from script {}",
        elements.len(),
        constructor.script_filename
    );

    let send_socket = UdpSocket::bind({
        let mut bind_addr = constructor.osc_socket_address;
        bind_addr.set_port(0);
        bind_addr
    })
    .await?;
    send_socket.connect(constructor.osc_socket_address).await?;

    loop {
        let now: LuaDateTime = OffsetDateTime::now_local()?.into();
        let messages: Vec<_> = elements
            .iter()
            .map(|e| call_value_function(&lua, e, now))
            .collect::<Result<_>>()?;

        for message in messages {
            send_socket.send(&message.serialize()).await?;
        }

        sleep(constructor.interval).await;
    }
}

/// Constructs conversion elements from Lua table.
async fn convert_table<'lua>(filename: &str) -> Result<(Lua, Vec<ConversionElement>)> {
    let script = read_to_string(filename).await?;
    let lua = Lua::new();
    let elements_table: LuaTable = lua.load(&script).call(())?;

    let globals = lua.globals();
    let mut elements = vec![];
    for pair in elements_table.pairs::<LuaValue, LuaValue>() {
        let (key, value) = pair?;
        let info_table = match value {
            LuaValue::Table(t) => t,
            _ => bail!("Convertion element must have a table value"),
        };

        let name = match key {
            LuaValue::String(s) => s.to_str()?.to_string(),
            LuaValue::Integer(i) => i.to_string(),
            LuaValue::Number(f) => f.to_string(),
            _ => bail!("Invalid key type: {:?}", key),
        };
        let address = info_table.get("address")?;
        let value_type = info_table
            .get::<_, Option<String>>("value_type")?
            .map(|s| s.parse())
            .transpose()?;
        let value_function_name = format!("vrcosc_clock_{}", name);
        let value_function: LuaFunction = info_table.get("value_function")?;
        globals.set(&value_function_name[..], value_function)?;

        elements.push(ConversionElement {
            address,
            value_type,
            value_function_name,
        });
    }
    drop(globals);

    Ok((lua, elements))
}

fn call_value_function(
    lua: &Lua,
    element: &ConversionElement,
    now: LuaDateTime,
) -> Result<OscMessage> {
    let globals = lua.globals();
    let function: LuaFunction = globals.get(&element.value_function_name[..])?;
    let value = function.call(now)?;

    let osc_value = into_osc_value(&value, element.value_type)?;
    let message = OscMessageBuilder::new(&element.address)?
        .push_argument(osc_value)
        .build();
    Ok(message)
}

/// Converts `LuaValue` to `OscValue` with type hinting.
fn into_osc_value(lua_value: &LuaValue, type_hint: Option<TargetType>) -> Result<OscValue> {
    let value = match (lua_value, type_hint) {
        (LuaValue::Integer(v), Some(TargetType::Int) | None) => OscValue::Int32(*v as i32),
        (LuaValue::Integer(v), Some(TargetType::Float)) => OscValue::Float32(*v as f32),
        (LuaValue::Number(v), Some(TargetType::Float) | None) => OscValue::Float32(*v as f32),
        (LuaValue::Number(v), Some(TargetType::Int)) => OscValue::Int32(*v as i32),
        _ => bail!("Invalid value conversion"),
    };
    Ok(value)
}
