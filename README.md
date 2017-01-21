# Rust WPILib [![Build Status](https://travis-ci.org/robotrs/rust-wpilib.svg?branch=master)](https://travis-ci.org/robotrs/rust-wpilib)

rust-wpilib is an equivalent to [WPILib](https://www.github.com/wpilibsuite/allwpilib), rewritten in Rust. The goal of
this project is to create a lightweight alternative to WPILib for FIRST Robotics Competition teams who would prefer to
use the language for its safety guarantees.

## Getting started
This repository is designed to be compiled for a [RoboRIO](http://sine.ni.com/nips/cds/view/p/lang/en/nid/213308), the
processor used in the FIRST Robotics Competition. To cross compile for RoboRIO, you have to do a few things:
 1. Install [Rustup](https://www.rustup.rs/) to help manage Rust toolchains.
 2. Run `rustup target add arm-unknown-linux-gnueabi` to install the Rust stdlib for ARM-based Linux.
 3. Install some variant of `arm-linux-gnueabi-gcc`. For example, the official FRC toolchain
    (`arm-frc-linux-gnueabi-gcc`) is available [here](https://launchpad.net/~wpilib/+archive/ubuntu/toolchain), or you
    can install a generic toolchain with your package manager of choice (`sudo apt-get install gcc-arm-linux-gnueabi` on
    Ubuntu).
 4. Edit your `~/.cargo/config` file with the following information:

    ```toml
    [target.arm-unknown-linux-gnueabi]
    linker = "<path-to-arm-linux-gnueabi-gcc>"
    ```

## Using this library as a dependency:
This library is not currently published on [crates.io](https://crates.io). However, you can directly declare it as a
dependency:

Cargo.toml:
```toml
[dependencies]
wpilib = { git = "https://github.com/robotrs/rust-wpilib" }
```

## Building
You can build your own project for the RoboRIO by passing `--target=arm-unknown-linux-gnueabi` when building.

To build this library on its own, just run `cargo build --target=arm-unknown-linux-gnueabi`. To make sure that
everything links properly, you may also want to run `cargo build --example test_robot
--target=arm-unknown-linux-gnueabi`.
