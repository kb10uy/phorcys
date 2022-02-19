# Phorcys
[Open Sound Control (OSC) implementation](https://opensoundcontrol.stanford.edu/) and [VRChat OSC API](https://docs.vrchat.com/v2022.1.1/docs/osc-overview) tools written in Rust!

## phorcys-osc
`phorcys-osc` provides simple abstraction for OSC packet.

## phorcys-config
`phorcys-config` defines configuration JSON types.

## phorcys-examples
`phorcys-examples` contains some example application using crates above.
You can try them like this: `cargo run --example monitor`.

## phorcys-miditable
`phorcys-miditable` converts MIDI message into OSC packets using VRChat's avatar configuration JSON and TOML-defined parameter table.
