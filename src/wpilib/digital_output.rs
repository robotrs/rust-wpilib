use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::sensor;
use std::ptr;

pub struct DigitalOutput {
    channel: i32,
    handle: HAL_DigitalHandle,
    pwm: Option<HAL_DigitalPWMHandle>,
}

impl DigitalOutput {
    pub fn new(channel: i32) -> HalResult<DigitalOutput> {
        if !sensor::check_digital_channel(channel) {
            return Err(0);
        }

        let handle = hal_call!(HAL_InitializeDIOPort(HAL_GetPort(channel), false as i32))?;
        unsafe {
            HAL_Report(14, channel, 0, ptr::null());
        }
        Ok(DigitalOutput {
            channel: channel,
            handle: handle,
            pwm: None,
        })
    }

    pub fn set(&mut self, value: bool) -> HalResult<()> {
        hal_call!(HAL_SetDIO(self.handle, value as i32))
    }

    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetDIO(self.handle))? != 0)
    }

    pub fn get_channel(&self) -> i32 {
        self.channel
    }

    pub fn get_handle(&self) -> HAL_DigitalHandle {
        self.handle
    }

    pub fn pulse(&mut self, length: f64) -> HalResult<()> {
        hal_call!(HAL_Pulse(self.handle, length))
    }

    pub fn is_pulsing(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_IsPulsing(self.handle))? != 0)
    }

    pub fn set_pwm_rate(&mut self, rate: f64) -> HalResult<()> {
        hal_call!(HAL_SetDigitalPWMRate(rate))
    }

    pub fn enable_pwm(&mut self, initial_duty_cycle: f64) -> HalResult<()> {
        let pwm = hal_call!(HAL_AllocateDigitalPWM())?;
        hal_call!(HAL_SetDigitalPWMDutyCycle(pwm, initial_duty_cycle))?;
        hal_call!(HAL_SetDigitalPWMOutputChannel(pwm, self.channel))?;
        self.pwm = Some(pwm);
        Ok(())
    }

    pub fn disable_pwm(&mut self) -> HalResult<()> {
        if let Some(pwm) = self.pwm {
            hal_call!(HAL_SetDigitalPWMOutputChannel(pwm, sensor::num_digital_channels()))?;
            hal_call!(HAL_FreeDigitalPWM(pwm))?;
            self.pwm = None;
        }
        Ok(())
    }

    pub fn update_duty_cycle(&mut self, duty_cycle: f64) -> HalResult<()> {
        if let Some(pwm) = self.pwm {
            hal_call!(HAL_SetDigitalPWMDutyCycle(pwm, duty_cycle))
        } else {
            Ok(())
        }
    }
}

impl Drop for DigitalOutput {
    fn drop(&mut self) {
        let _ = self.disable_pwm();
        unsafe {
            HAL_FreeDIOPort(self.handle);
        }
    }
}
