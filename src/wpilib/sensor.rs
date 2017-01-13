use wpilib::wpilib_hal::*;

/// The result of an interrupt wait
pub enum WaitResult {
    /// Waiting on the interrupt timed out.
    Timeout = 0x0,
    /// The interrupt was triggered by a rising edge.
    RisingEdge = 0x1,
    /// The interrupt was triggered by a falling edge.
    FallingEdge = 0x100,
    /// The interrupt was triggered by both a rising edge and a falling edge.
    Both = 0x101,
}

/// The number of DIOs on the RoboRIO.
pub fn num_digital_channels() -> i32 {
    unsafe { HAL_GetNumDigitalChannels() }
}

/// The number of analog inputs on the RoboRIO.
pub fn num_analog_inputs() -> i32 {
    unsafe { HAL_GetNumAnalogInputs() }
}

/// The number of solenoid channels per PCM.
pub fn num_solenoid_channels() -> i32 {
    unsafe { HAL_GetNumSolenoidChannels() }
}

/// The number of PCMs.
pub fn num_solenoid_modules() -> i32 {
    unsafe { HAL_GetNumPCMModules() }
}

/// The number of hardware PWM channels on the RoboRIO.
pub fn num_pwm_channels() -> i32 {
    unsafe { HAL_GetNumPWMChannels() }
}

/// The number of relay headers on the RoboRIO.
pub fn num_relay_headers() -> i32 {
    unsafe { HAL_GetNumRelayHeaders() }
}

/// Check if a solenoid module is valid.
pub fn check_solenoid_module(module: i32) -> bool {
    unsafe { HAL_CheckSolenoidModule(module) != 0 }
}

/// Check if a digital channel is valid.
pub fn check_digital_channel(channel: i32) -> bool {
    unsafe { HAL_CheckDIOChannel(channel) != 0 }
}

/// Check if a relay channel is valid.
pub fn check_relay_channel(channel: i32) -> bool {
    unsafe { HAL_CheckRelayChannel(channel) != 0 }
}

/// Check if a PWM channel is valid.
pub fn check_pwm_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPWMChannel(channel) != 0 }
}

/// Check if an analog input channel is valid.
pub fn check_analog_input_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogInputChannel(channel) != 0 }
}

/// Check if an analog output channel is valid.
pub fn check_analog_output_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogOutputChannel(channel) != 0 }
}

/// Check if a solenoid channel is valid.
pub fn check_solenoid_channel(channel: i32) -> bool {
    unsafe { HAL_CheckSolenoidChannel(channel) != 0 }
}

/// Check if a PDP channel is valid.
pub fn check_pdp_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPDPModule(channel) != 0 }
}
