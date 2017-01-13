use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::sensor;

/// An interface to the PDP for getting information about robot power.
pub struct PowerDistributionPanel {
    module: i32,
}

impl PowerDistributionPanel {
    /// Create a new PDP interface on the specified module.
    pub fn new(module: i32) -> HalResult<PowerDistributionPanel> {
        hal_call!(HAL_InitializePDP(module))?;
        Ok(PowerDistributionPanel { module: module })
    }

    /// Use the default module (0).
    pub fn default() -> HalResult<PowerDistributionPanel> {
        PowerDistributionPanel::new(0)
    }

    /// Get the voltage going into the PDP.
    pub fn get_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPVoltage(self.module))
    }

    /// Get the PDP's temperature, in degrees Celsius.
    pub fn get_temperature(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTemperature(self.module))
    }

    /// Get the current on a specific channel on the PDP, in amps.
    pub fn get_current(&self, channel: i32) -> HalResult<f64> {
        if !sensor::check_pdp_channel(channel) {
            return Err(0);
        }

        hal_call!(HAL_GetPDPChannelCurrent(self.module, channel))
    }

    /// Get the total current drawn from the PDP, in amps.
    pub fn get_total_current(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTotalCurrent(self.module))
    }

    /// Get the total power drawn from the PDP, in watts.
    pub fn get_total_power(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTotalPower(self.module))
    }

    /// Get the total energy expended by the PDP, in joules.
    pub fn get_total_energy(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTotalEnergy(self.module))
    }

    /// Reset the total energy count so far to zero.
    pub fn reset_total_energy(&mut self) -> HalResult<()> {
        hal_call!(HAL_ResetPDPTotalEnergy(self.module))
    }

    /// Clear sticky faults in the PDP.
    pub fn clear_sticky_faults(&mut self) -> HalResult<()> {
        hal_call!(HAL_ClearPDPStickyFaults(self.module))
    }
}
