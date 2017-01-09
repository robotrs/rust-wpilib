use wpilib::wpilib_hal::*;

pub enum WaitResult {
    Timeout = 0x0,
    RisingEdge = 0x1,
    FallingEdge = 0x100,
    Both = 0x101,
}

pub fn num_digital_channels() -> i32 {
    unsafe { HAL_GetNumDigitalChannels() }
}

pub fn num_analog_inputs() -> i32 {
    unsafe { HAL_GetNumAnalogInputs() }
}

pub fn num_solenoid_channels() -> i32 {
    unsafe { HAL_GetNumSolenoidChannels() }
}

pub fn num_solenoid_modules() -> i32 {
    unsafe { HAL_GetNumPCMModules() }
}

pub fn num_pwm_channels() -> i32 {
    unsafe { HAL_GetNumPWMChannels() }
}

pub fn num_relay_headers() -> i32 {
    unsafe { HAL_GetNumRelayHeaders() }
}

pub fn check_solenoid_module(module: i32) -> bool {
    unsafe { HAL_CheckSolenoidModule(module) != 0 }
}

pub fn check_digital_channel(channel: i32) -> bool {
    unsafe { HAL_CheckDIOChannel(channel) != 0 }
}

pub fn check_relay_channel(channel: i32) -> bool {
    unsafe { HAL_CheckRelayChannel(channel) != 0 }
}

pub fn check_pwm_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPWMChannel(channel) != 0 }
}

pub fn check_analog_input_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogInputChannel(channel) != 0 }
}

pub fn check_analog_output_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogOutputChannel(channel) != 0 }
}

pub fn check_solenoid_channel(channel: i32) -> bool {
    unsafe { HAL_CheckSolenoidChannel(channel) != 0 }
}

pub fn check_pdp_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPDPModule(channel) != 0 }
}
