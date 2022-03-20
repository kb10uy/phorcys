local function convert_hour(datetime)
    local seconds = datetime.hour * 3600 + datetime.minute * 60 + datetime.second % 43200;
    return seconds / 43200.0;
end

local function convert_minute(datetime)
    local seconds = datetime.minute * 60 + datetime.second;
    return seconds / 3600.0;
end

local function convert_second(datetime)
    local seconds = datetime.second;
    return seconds / 60.0;
end

return {
    hour = {
        address = "/address/to/hour",
        value_type = "float",
        value_function = convert_hour,
    },
    minute = {
        address = "/address/to/hour",
        value_type = "float",
        value_function = convert_hour,
    },
    second = {
        address = "/address/to/hour",
        value_type = "float",
        value_function = convert_hour,
    },
};
