#![allow(missing_docs)]

use std::ffi;

use wpilib::wpilib_hal::*;
use wpilib::driverstation::*;

const RUMBLE_BASE: i32 = 65535;

#[derive(PartialEq)]
pub enum JoystickSide {
    LeftHand,
    RightHand,
}

#[derive(PartialEq)]
pub enum DPad {
    Neutral,
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

pub trait JoystickBase {
    fn get_raw_axis(&mut self, axis: usize) -> Result<f32, JoystickError>;
    fn get_raw_button(&mut self, button: usize) -> Result<bool, JoystickError>;
    fn get_pov(&mut self, pov: usize) -> Result<i16, JoystickError>;
    fn set_output(&mut self, output_number: i32, value: bool);
    fn set_outputs(&mut self, value: i64);
    fn set_rumble(&mut self, side: JoystickSide, value: f32);
}

pub struct Joystick {
    port: usize,
    ds: &'static mut DriverStation,
    outputs: i64,
    left_rumble: i32,
    right_rumble: i32,
}

impl Joystick {
    pub fn new(p: usize) -> Joystick {
        Joystick { port: p, ds: DriverStation::instance(), outputs: 0i64, left_rumble: 0i32, right_rumble: 0i32 }
    }
}

impl JoystickBase for Joystick {
    fn get_raw_axis(&mut self, axis: usize) -> Result<f32, JoystickError> {
        self.ds.get_joystick_axis(self.port, axis)
    }

    fn get_raw_button(&mut self, button: usize) -> Result<bool, JoystickError> {
        self.ds.get_joystick_button(self.port, button)
    }

    fn get_pov(&mut self, pov: usize) -> Result<i16, JoystickError> {
        self.ds.get_joystick_pov(self.port, pov)
    }

    fn set_output(&mut self, output_number: i32, value: bool) {
        let o = output_number - 1i32;
        self.outputs = (self.outputs & (!(1i32 << o)) as i64) | ((value as i64) << o);
        unsafe { HAL_SetJoystickOutputs(self.port as i32, self.outputs, self.left_rumble, self.right_rumble); }
    }

    fn set_outputs(&mut self, value: i64) {
        self.outputs = value;
        unsafe { HAL_SetJoystickOutputs(self.port as i32, self.outputs, self.left_rumble, self.right_rumble); }
    }

    fn set_rumble(&mut self, side: JoystickSide, mut value: f32) {
        value = if value > 1f32 { 1f32 } else if value < 0f32 { 0f32 } else { value };
        match side {
            JoystickSide::LeftHand => self.left_rumble = (value * RUMBLE_BASE as f32) as i32,
            JoystickSide::RightHand => self.right_rumble = (value * RUMBLE_BASE as f32) as i32,
        }
        unsafe { HAL_SetJoystickOutputs(self.port as i32, self.outputs, self.left_rumble, self.right_rumble) };
    }
}

pub struct XBoxController {
    port: usize,
    ds: &'static mut DriverStation,
    outputs: i64,
    left_rumble: i32,
    right_rumble: i32,
}

impl XBoxController {
    pub fn new(p: usize) -> XBoxController {
        XBoxController { port: p, ds: DriverStation::instance(), outputs: 0i64, left_rumble: 0i32, right_rumble: 0i32 }
    }

    //buttons
    pub fn get_a_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(1)
    }

    pub fn get_b_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(2)
    }

    pub fn get_x_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(3)
    }

    pub fn get_y_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(4)
    }

    pub fn get_bumper(&mut self, side: JoystickSide) -> Result<bool, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_button(5),
            JoystickSide::RightHand => self.get_raw_button(6),
        }
    }

    pub fn get_back_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(7)
    }

    pub fn get_start_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(8)
    }

    pub fn get_axis_button(&mut self, side: JoystickSide) -> Result<bool, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_button(9),
            JoystickSide::RightHand => self.get_raw_button(10),
        }
    }

    //axes
    pub fn get_x(&mut self, side: JoystickSide) -> Result<f32, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_axis(0),
            JoystickSide::RightHand => self.get_raw_axis(4),
        }
    }

    pub fn get_y(&mut self, side: JoystickSide) -> Result<f32, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_axis(1),
            JoystickSide::RightHand => self.get_raw_axis(5),
        }
    }

    pub fn get_trigger(&mut self, side: JoystickSide) -> Result<f32, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_axis(2),
            JoystickSide::RightHand => self.get_raw_axis(3),
        }
    }

    pub fn get_dpad(&mut self) -> Result<DPad, JoystickError> {
        match self.get_pov(1) {
            Ok(val) if val == 0 => Ok(DPad::Up),
            Ok(val) if val == 1 => Ok(DPad::UpRight),
            Ok(val) if val == 2 => Ok(DPad::Right),
            Ok(val) if val == 3 => Ok(DPad::DownRight),
            Ok(val) if val == 4 => Ok(DPad::Down),
            Ok(val) if val == 5 => Ok(DPad::DownLeft),
            Ok(val) if val == 6 => Ok(DPad::Left),
            Ok(val) if val == 7 => Ok(DPad::UpLeft),
            Ok(val) if val == -01 => Ok(DPad::Neutral),
            Ok(_) => {
                let mut message: String = String::from("Received invalid POV value on XBox controller at port ");
                message += self.port.to_string().as_str();
                unsafe {
                    HAL_SendError(false as i32,
                                  1,
                                  false as i32,
                                  ffi::CString::new(message.as_str()).unwrap().into_raw(),
                                  ffi::CString::new("").unwrap().into_raw(),
                                  ffi::CString::new("").unwrap().into_raw(),
                                  true as i32);
                }
                Ok(DPad::Neutral)
            },
            Err(e) => Err(e),
        }
    }
 }

impl JoystickBase for XBoxController {
    fn get_raw_axis(&mut self, axis: usize) -> Result<f32, JoystickError> {
        self.ds.get_joystick_axis(self.port, axis)
    }

    fn get_raw_button(&mut self, button: usize) -> Result<bool, JoystickError> {
        self.ds.get_joystick_button(self.port, button)
    }

    fn get_pov(&mut self, pov: usize) -> Result<i16, JoystickError> {
        self.ds.get_joystick_pov(self.port, pov)
    }

    fn set_output(&mut self, output_number: i32, value: bool) {
        let o = output_number - 1i32;
        self.outputs = (self.outputs & (!(1i32 << o)) as i64) | ((value as i64) << o);
        unsafe { HAL_SetJoystickOutputs(self.port as i32, self.outputs, self.left_rumble, self.right_rumble); }
    }

    fn set_outputs(&mut self, value: i64) {
        self.outputs = value;
        unsafe { HAL_SetJoystickOutputs(self.port as i32, self.outputs, self.left_rumble, self.right_rumble); }
    }

    fn set_rumble(&mut self, side: JoystickSide, mut value: f32) {
        value = if value > 1f32 { 1f32 } else if value < 0f32 { 0f32 } else { value };
        match side {
            JoystickSide::LeftHand => self.left_rumble = (value * RUMBLE_BASE as f32) as i32,
            JoystickSide::RightHand => self.right_rumble = (value * RUMBLE_BASE as f32) as i32,
        }
        unsafe { HAL_SetJoystickOutputs(self.port as i32, self.outputs, self.left_rumble, self.right_rumble) };
    }
}
