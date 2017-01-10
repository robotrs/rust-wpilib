use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::sensor;
use std::ptr;

pub struct DigitalInput {
    channel: i32,
    handle: HAL_DigitalHandle,
}

impl DigitalInput {
    pub fn new(channel: i32) -> HalResult<DigitalInput> {
        if !sensor::check_digital_channel(channel) {
            return Err(0);
        }

        let handle = hal_call!(HAL_InitializeDIOPort(HAL_GetPort(channel), true as i32))?;
        unsafe {
            HAL_Report(13, channel, 0, ptr::null());
        }
        Ok(DigitalInput {
            channel: channel,
            handle: handle,
        })
    }

    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetDIO(self.handle))? != 0)
    }

    pub fn get_channel(&self) -> i32 {
        self.channel
    }

    pub fn get_handle(&self) -> HAL_Handle {
        self.handle
    }
}

impl Drop for DigitalInput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDIOPort(self.handle);
        }
    }
}
