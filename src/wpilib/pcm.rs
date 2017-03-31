use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::usage::*;
use wpilib::can::CanError;
use wpilib::sensor;
use std::ptr;
use std::sync;

/// A single-acting solenoid that flushes immediately on write.
pub struct Solenoid {
    channel: i32,
    module: i32,
    handle: HAL_SolenoidHandle,
}

/// An error in the creation of a solenoid
#[derive(Debug, Copy, Clone)]
pub enum SolenoidCreationError {
    /// The specified module does not exist
    ModuleDNE,
    /// The specified channel is invalid or does not exist
    ChannelDNE,
    /// Some other HAL error
    Other(HalError),
}

impl Solenoid {
    /// Create a new solenoid on the specified module, using the default PCM module 0.
    pub fn new(channel: i32) -> Result<Solenoid, SolenoidCreationError> {
        Solenoid::new_with_module(0, channel)
    }

    /// Create a new solenoid on the specified module and channel.
    pub fn new_with_module(module: i32, channel: i32) -> Result<Solenoid, SolenoidCreationError> {
        if !sensor::check_solenoid_module(module) {
            return Err(SolenoidCreationError::ModuleDNE);
        }

        if !sensor::check_solenoid_channel(channel) {
            return Err(SolenoidCreationError::ChannelDNE);
        }

        let handle = hal_call!(HAL_InitializeSolenoidPort(HAL_GetPortWithModule(module, channel)))
            .map_err(|e| SolenoidCreationError::Other(e))?;

        report_usage_extras(ResourceType::Solenoid, channel, module, ptr::null());

        Ok(Solenoid {
            channel: channel,
            module: module,
            handle: handle,
        })
    }

    /// Set the value of the solenoid, flushing immediately to CAN.
    pub fn set(&mut self, value: bool) -> Result<(), CanError> {
        hal_call!(HAL_SetSolenoid(self.handle, value as i32)).map_err(From::from)
    }

    /// Get the most recently set value of the solenoid.
    pub fn get(&self) -> Result<bool, CanError> {
        hal_call!(HAL_GetSolenoid(self.handle)).map(|b| b != 0).map_err(From::from)
    }

    /// Check if the solenoid has been blacklisted by the PCM. A solenoid is blacklisted when it
    /// becomes shorted, and is not removed from the PCM's blacklist until sticky faults are
    /// cleared or the robot is rebooted.
    pub fn is_blacklisted(&self) -> Result<bool, CanError> {
        match hal_call!(HAL_GetPCMSolenoidBlackList(self.module)) {
            Ok(blacklist) => Ok((blacklist & (1 << self.channel)) != 0),
            Err(e) => Err(From::from(e)),
        }
    }
}

impl Drop for Solenoid {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeSolenoidPort(self.handle);
        }
    }
}
