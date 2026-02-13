# MyCobot Rust Library

[![Crates.io](https://img.shields.io/crates/v/mycobot-rs.svg)](https://crates.io/crates/mycobot-rs)
[![Documentation](https://docs.rs/mycobot-rs/badge.svg)](https://docs.rs/mycobot-rs)
[![License](https://img.shields.io/crates/l/mycobot-rs.svg)](LICENSE)
[![Publish](https://github.com/progress-robotics/mycobot-rs/actions/workflows/publish.yml/badge.svg)](https://github.com/progress-robotics/mycobot-rs/actions/workflows/publish.yml)
[![Build](https://github.com/progress-robotics/mycobot-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/progress-robotics/mycobot-rs/actions/workflows/rust.yml)

A Rust implementation of the MyCobot communication protocol. This library provides a safe and idiomatic Rust interface for controlling Elephant Robotics' MyCobot arms.

## Features

- **Protocol Implementation**: Complete implementation of the MyCobot serial communication protocol.
- **Motion Control**: 
  - Get and set joint angles.
  - Get and set coordinates (XYZ + RxRyRz).
- **Basic Control**:
  - Power on/off.
  - Check power state.
  - Set Atom LED color.
- **Safety**: Error handling for I/O and protocol issues.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mycobot-rs = "0.1.0"
```

Or run:

```bash
cargo add mycobot-rs
```

## Usage

Here is a basic example of how to connect to the robot, power it on, and read the joint angles. This example is available in `examples/basic.rs`.

```rust
use mycobot_rs::MyCobot;
use serial2::SerialPort as SysSerial;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = SysSerial::open("/dev/ttyAMA0", 1_000_000)?;
    let mut robot = MyCobot::new(port);

    robot.power_on()?;

    let angles = robot.get_angles()?;
    println!("Angles: {:?}", angles);

    Ok(())
}
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Examples

To run the examples, use:

```bash
cargo run --example basic
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
