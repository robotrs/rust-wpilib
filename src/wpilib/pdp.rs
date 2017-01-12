use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::sensor;

pub struct PowerDistributionPanel {
    module: i32,
}

impl PowerDistributionPanel {
    fn new(module: i32) -> HalResult<PowerDistributionPanel> {
        hal_call!(HAL_InitializePDP(module))?;
        Ok(PowerDistributionPanel { module: module })
    }

    fn default() -> HalResult<PowerDistributionPanel> {
        PowerDistributionPanel::new(0)
    }

    fn get_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPVoltage(self.module))
    }

    fn get_temperature(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTemperature(self.module))
    }

    fn get_current(&self, channel: i32) -> HalResult<f64> {
        if !sensor::check_pdp_channel(channel) {
            return Err(HalError(0));
        }

        hal_call!(HAL_GetPDPChannelCurrent(self.module, channel))
    }

    fn get_total_current(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTotalCurrent(self.module))
    }

    fn get_total_power(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTotalPower(self.module))
    }

    fn get_total_energy(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTotalEnergy(self.module))
    }

    fn reset_total_energy(&mut self) -> HalResult<()> {
        hal_call!(HAL_ResetPDPTotalEnergy(self.module))
    }

    fn clear_sticky_faults(&mut self) -> HalResult<()> {
        hal_call!(HAL_ClearPDPStickyFaults(self.module))
    }
}
