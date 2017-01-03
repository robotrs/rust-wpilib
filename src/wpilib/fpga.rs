use wpilib::wpilib_hal as hal;

/// Gets the FPGA version
pub fn version() -> i32 {
    let mut status = 0i32;
    unsafe {
        hal::HAL_GetFPGAVersion(&mut status as *mut i32)
    }
}

/// Gets the FPGA revision
pub fn revision() -> i64 {
    let mut status = 0i32;
    unsafe {
        hal::HAL_GetFPGARevision(&mut status as *mut i32)
    }
}

/// Gets the FPGA time in microseconds since the FPGA reset. Rolls over after 71 minutes.
pub fn get_time_us() -> u64 {
    let mut status = 0i32;
    unsafe {
        hal::HAL_GetFPGATime(&mut status as *mut i32)
    }
}

pub fn user_button_down() -> bool {
    let mut status = 0i32;
    unsafe {
        hal::HAL_GetFPGAButton(&mut status as *mut i32) != 0
    }
}
