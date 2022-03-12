# vrcosc-clock
Sends DateTime information to VRChat as OSC packets.

## Structure
`vrcosc-clock` breaks a DateTime information into several "parts".
Each of them expresses a single part like hour, minute, and second.

`vrcosc-clock` can send a part in two formats: absolute form and relative form.
* An **Absolute part** will be send as integer, which value is the remainder divided by the `divider` parameter.
    - e.g. Hour parameter is from 0 to 23. If you set `divider` as 12, sent value will be 0 to 11 (AM/PM).
* A **Relative part** will be send as float, which value is the ratio between original value and max value. It may be multiplied by `max_value` parameter.
    - e.g. Second parameter is from 0 to 59. If you set `max_value` as 1.0, sent value will be 0/60 to 59/60. If `max_value` is 0.6, sent value will be 0.0 to 0.59.

## Usage
There are two ways to tell which part `vrcosc-clock` should send to VRChat.

### 1. Specify in commandline options
```sh
vrcosc-clock -i 2         \
  -h /clock/hour,rel,12   \
  -m /clock/minute,rel,60 \
  -s /clock/second,rel,60
```

For each part option, you must give the information string as:
1. Absolute part: `/path/to/param,abs,<divider>`
2. Relative part: `/path/to/param,rel,<max_value>`

### 2. Specify in TOML file
Create a TOML file like below:
```toml
# Allowed table names are:
# hour, minute, second, month, day

# Absolute part example
[minute]
path = "/path/to/parameter"
format = "absolute"
divider = 60

# Relative part example
[second]
path = "/path/to/parameter"
format = "relative"
max_value = 1.0
```

And just give it to vrcosc-clock:
```sh
vrcosc-clock -i 2 parts.toml
```

## License
This project is licensed under Apache License 2.0.
