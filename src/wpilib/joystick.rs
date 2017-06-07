use wpilib::wpilib_hal::*;
use wpilib::driverstation::*;

const RUMBLE_BASE: i32 = 65535;

/// Enum for accessing elements of XBox controller by side
#[derive(PartialEq)]
pub enum JoystickSide {
    /// left side of joystick while held upright
    LeftHand,
    /// right side of joystick while held upright
    RightHand,
}

/// Enum for state of XBox controller POV
#[derive(PartialEq)]
pub enum DPad {
    /// no value pressed
    Neutral,
    /// equivalent to up arrow
    Up,
    /// equivalent to down arrow
    Down,
    /// equivalent to left arrow
    Left,
    /// equivalent to right arrow
    Right,
    /// equivalent to upward diagonal right arrow
    UpRight,
    /// equivalent to downward diagonal right arrow
    DownRight,
    /// equivalent to upward diagonal left arrow
    UpLeft,
    /// equivalent to downward diagonal right arrow
    DownLeft,
    /// case for if plugged in joystick returns a pov value that is not valid on a standard XBox controller
    /// should only be invoked if a different kind of joystick is plugged into the port the XBoxController struct
    /// is set to
    Invalid,
}

/// public trait that lays down base methods for joysticks
pub trait JoystickBase {
    /// get raw axis value from driverstation
    fn get_raw_axis(&mut self, axis: usize) -> Result<f32, JoystickError>;
    /// get raw button value from driverstation
    fn get_raw_button(&mut self, button: usize) -> Result<bool, JoystickError>;
    /// get raw pov value from driverstation
    fn get_pov(&mut self, pov: usize) -> Result<i16, JoystickError>;
    /// set joystick output through hal
    fn set_output(&mut self, output_number: i32, value: bool);
    /// set joystick outputs through hal
    fn set_outputs(&mut self, value: i64);
    /// set joystick rumble on either side by a percentage from 0-100 through hal
    fn set_rumble(&mut self, side: JoystickSide, value: f32);
}

/// stuct for almost any FRC legal joystick
pub struct Joystick {
    port: usize,
    ds: &'static mut DriverStation,
    outputs: i64,
    left_rumble: i32,
    right_rumble: i32,
}

impl Joystick {
    /// user creates a Joystick object here
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

/// helper struct for teams that use a standard XBox 360 controller, similar in practice to a generic joystick
pub struct XBoxController {
    port: usize,
    ds: &'static mut DriverStation,
    outputs: i64,
    left_rumble: i32,
    right_rumble: i32,
}

impl XBoxController {
    /// users create an XBox controller object here
    pub fn new(p: usize) -> XBoxController {
        XBoxController { port: p, ds: DriverStation::instance(), outputs: 0i64, left_rumble: 0i32, right_rumble: 0i32 }
    }

    /// simply wrappers for get_raw_[axis/button] with hardcoded values for the standard xbox 360 controller

    /// buttons

    /// helper function for getting a button
    pub fn get_a_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(1)
    }

    /// helper function for getting b button
    pub fn get_b_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(2)
    }

    /// helper function for getting x button
    pub fn get_x_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(3)
    }

    /// helper function for getting y button
    pub fn get_y_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(4)
    }

    /// helper function for getting a bumper button
    pub fn get_bumper(&mut self, side: JoystickSide) -> Result<bool, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_button(5),
            JoystickSide::RightHand => self.get_raw_button(6),
        }
    }

    /// helper function for getting back button
    pub fn get_back_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(7)
    }

    /// helper function for getting start button
    pub fn get_start_button(&mut self) -> Result<bool, JoystickError> {
        self.get_raw_button(8)
    }

    /// helper function for getting an axis button
    pub fn get_axis_button(&mut self, side: JoystickSide) -> Result<bool, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_button(9),
            JoystickSide::RightHand => self.get_raw_button(10),
        }
    }

    /// axes

    /// helper function for getting an x axis
    pub fn get_x(&mut self, side: JoystickSide) -> Result<f32, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_axis(0),
            JoystickSide::RightHand => self.get_raw_axis(4),
        }
    }

    /// helper function for getting a y axis
    pub fn get_y(&mut self, side: JoystickSide) -> Result<f32, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_axis(1),
            JoystickSide::RightHand => self.get_raw_axis(5),
        }
    }

    /// helper function for getting a trigger
    pub fn get_trigger(&mut self, side: JoystickSide) -> Result<f32, JoystickError> {
        match side {
            JoystickSide::LeftHand => self.get_raw_axis(2),
            JoystickSide::RightHand => self.get_raw_axis(3),
        }
    }

    /// helper function for getting dpad position, returns a DPad enum with an actual direction for ease of use and will
    /// print a warning if the value returned by get_pov is not a valid pov value for the xbox 360 controller
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
                self.ds.report_throttled(false, message.as_str());
                Ok(DPad::Invalid)
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
