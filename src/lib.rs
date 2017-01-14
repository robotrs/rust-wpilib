//! # Rust WPILib
//! This is a port of the WPILib library for FIRST Robotics Competition (FRC) to Rust. It is still
//! very much a work in progress, and much of its functionality is untested. It is *not
//! recommmended* to use this library in a competition setting.

#![allow(dead_code)]
#![deny(missing_docs)]

extern crate atom;

#[cfg(target_arch = "arm")]
#[macro_use]
mod athena;

#[cfg(target_arch = "arm")]
pub use athena::*;

#[cfg(not(target_arch = "arm"))]
mod sim;

#[cfg(not(target_arch = "arm"))]
pub use sim::*;
