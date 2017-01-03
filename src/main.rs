extern crate wpilib;
use wpilib::*;

fn main() {
    unsafe{ let _ = wpilib::wpilib_hal::HAL_Initialize(0); }
}
