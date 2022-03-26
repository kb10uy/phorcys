# vrcosc-clock
Sends DateTime information to VRChat as OSC packets using Lua script.

## Usage

### 1. Prepare a Lua script
```lua
-- Conversion function receives DateTime table like below:
-- { hour = 14, minute = 23, second = 42, month = 12, day = 31 }
-- time parts are 0-based, and date parts are 1-based.
-- The table is readonly.
local function convert_hour(datetime)
    return datetime.hour;
end

local function convert_minute(datetime)
    return datetime.minute;
end

-- The script must return a table.
-- Each element of the table must have specific structure described below.
-- You may name the keys of table for convenience in logging.
return {
    hour = {
        -- OSC Address to which vrcosc-clock should send values.
        address = "/clock/hour",

        -- OSC value type. It must be "int", "float", or unspecified.
        -- When specified, returned value from the function will be casted
        -- and sent as corresponding type (recommended).
        -- When unspecified, returned value will not be casted, so implicit type will be used.
        value_type = "float",

        -- Lua function to convert datetime into sending value.
        value_function = convert_hour,
    },
    minute = {
        address = "/clock/minute",
        value_type = "float",
        value_function = convert_minute,
    },
};
```

### 2. Execute `vrcosc-clock`
```sh
vrcosc-clock -i 2 script.lua
```

## License
This project is licensed under Apache License 2.0.
