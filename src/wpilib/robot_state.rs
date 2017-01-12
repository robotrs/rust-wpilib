use wpilib::wpilib_hal::*;

pub fn is_browned_out() -> HalResult<bool> {
    Ok(hal_call!(HAL_GetBrownedOut())? as bool)
}

pub fn is_system_active() -> HalResult<bool> {
    Ok(hal_call!(HAL_GetSystemActive())? as bool)
}

pub fn get_battery_voltage() -> HalResult<f64> {
    hal_call!(HAL_GetVinVoltage())
}
