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

/// A struct representing a PCM object that is buffered and stores values until it is flushed. This
/// object can create BufferedSolenoids to act as proxies for setting PCM values.
pub struct BufferedPcm {
    buffer: sync::Mutex<u8>,
    module: i32,
}

/// A single-acting solenoid that caches out values to a BufferedPcm instead of writing directly to
/// CAN.
pub struct BufferedSolenoid<'a> {
    buffer: &'a sync::Mutex<u8>,
    channel: i32,
}

impl BufferedPcm {
    /// Create a new BufferedPcm on the specified module, returning an Err if the module is invalid.
    pub fn new(module: i32) -> Result<BufferedPcm, SolenoidCreationError> {
        if !sensor::check_solenoid_module(module) {
            Err(SolenoidCreationError::ModuleDNE)
        } else {
            Ok(BufferedPcm {
                buffer: sync::Mutex::new(0),
                module: module,
            })
        }
    }

    /// Make a new BufferedSolenoid on this PCM on the specified channel, returning an Err if the
    /// channel is invalid.
    pub fn make_solenoid<'a>(&'a self,
                             channel: i32)
                             -> Result<BufferedSolenoid<'a>, SolenoidCreationError> {
        if !sensor::check_solenoid_channel(channel) {
            Err(SolenoidCreationError::ChannelDNE)
        } else {
            Ok(BufferedSolenoid {
                buffer: &self.buffer,
                channel: channel,
            })
        }
    }

    /// Flush the cached values to CAN.
    pub fn flush(&self) -> Result<(), CanError> {
        let data = *self.buffer.lock().unwrap();
        hal_call!(HAL_SetAllSolenoids(self.module, data as i32)).map_err(From::from)
    }
}

impl<'a> BufferedSolenoid<'a> {
    /// Set the value of the solenoid. This will cache values to the BufferedPcm, which will not
    /// actually write out values to CAN until flush() is called.
    pub fn set(&mut self, value: bool) {
        let mut data = self.buffer.lock().unwrap();
        if value {
            *data |= 1 << self.channel;
        } else {
            *data &= !(1 << self.channel);
        }
    }
}
