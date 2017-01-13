use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::sensor;
use wpilib::usage::*;

/// A digital output used to control lights, etc from the RoboRIO.
pub struct DigitalOutput {
    channel: i32,
    handle: HAL_DigitalHandle,
}

impl DigitalOutput {
    /// Create a new digital output on the specificed channel, returning an error if initialization
    /// fails.
    pub fn new(channel: i32) -> HalResult<DigitalOutput> {
        if !sensor::check_digital_channel(channel) {
            return Err(0);
        }

        let handle = hal_call!(HAL_InitializeDIOPort(HAL_GetPort(channel), false as i32))?;

        report_usage(ResourceType::DigitalOutput, channel);

        Ok(DigitalOutput {
            channel: channel,
            handle: handle,
        })
    }

    /// Set the value to output.
    pub fn set(&mut self, value: bool) -> HalResult<()> {
        hal_call!(HAL_SetDIO(self.handle, value as i32))
    }

    /// Get the previously-written output.
    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetDIO(self.handle))? != 0)
    }

    /// Get the channel for this DIO.
    pub fn get_channel(&self) -> i32 {
        self.channel
    }

    /// Get a handle to this DIO.
    pub fn get_handle(&self) -> HAL_DigitalHandle {
        self.handle
    }

    /// Write a pulse to this output.
    pub fn pulse(&mut self, length: f64) -> HalResult<()> {
        hal_call!(HAL_Pulse(self.handle, length))
    }

    /// Is this output currently in the middle of a pulse?
    pub fn is_pulsing(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_IsPulsing(self.handle))? != 0)
    }
}

impl Drop for DigitalOutput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDIOPort(self.handle);
        }
    }
}
