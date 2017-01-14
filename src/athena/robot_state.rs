use athena::wpilib_hal::*;
use athena::hal_call::*;

/// Is the robot browned out?
pub fn is_browned_out() -> HalResult<bool> {
    Ok(hal_call!(HAL_GetBrownedOut())? != 0)
}

/// Are outputs enabled? A result of false here could be caused by a disabled robot or a brownout.
pub fn is_system_active() -> HalResult<bool> {
    Ok(hal_call!(HAL_GetSystemActive())? != 0)
}

/// Get the robot's current battery voltage.
pub fn get_battery_voltage() -> HalResult<f64> {
    hal_call!(HAL_GetVinVoltage())
}
