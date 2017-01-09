use wpilib::wpilib_hal::*;

/// Gets the FPGA version
pub fn version() -> i32 {
    hal_call!(HAL_GetFPGAVersion()).unwrap()
}

/// Gets the FPGA revision
pub fn revision() -> i64 {
    hal_call!(HAL_GetFPGARevision()).unwrap()
}

/// Gets the FPGA time in microseconds since the FPGA reset. Rolls over after 71 minutes.
pub fn get_time_us() -> u64 {
    hal_call!(HAL_GetFPGATime()).unwrap()
}

pub fn user_button_down() -> bool {
    hal_call!(HAL_GetFPGAButton()).unwrap() != 0
}
