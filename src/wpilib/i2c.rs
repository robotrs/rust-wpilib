use wpilib::wpilib_hal::*;
use wpilib::usage::*;
use wpilib::hal_call::*;

/// Describes one of two possible locations on the roborio for i2c devices
#[derive(Copy, Clone)]
pub enum I2cPort {
    /// Enum option for i2c device plugged into the onboard i2c connectors on the roborio
    OnBoard = 0,
    /// Enum option for i2c device, such as a navX board, connected through the MXP port
    MXP = 1,
}

/// Describe various errored or otherwise special cases that may result from an i2c transaction
pub enum I2cError {
    // TODO find is there is any way to get more i2c transaction information out of HAl than just
    // success or failure

    /// Indicates a general error in an i2c transaction
    IOError,
    /// Indicates when no string, or an invalid string was received from the i2c transaction
    InvalidReceiveString,
    /// Indicates a situation where the connection was specifically aborted
    TransferAbort,
}

/// Struct for sending and receiving data over i2c
pub struct I2cInterface {
    port: I2cPort,
    device_address: i32,
}

impl I2cInterface {
    /// Create a new I2cInterface here
    pub fn new(p: I2cPort, addr: i32) -> HalResult<I2cInterface> {
        let mut init_status = 0;
        unsafe {
            HAL_InitializeI2C(p as i32, &mut init_status as *mut i32);
            report_usage(ResourceType::I2C, p as i32);
        }
        match init_status {
            0 => Ok(I2cInterface { port: p, device_address: addr }),
            _ => Err(HalError(0)),
        }
    }

    /// Perform a simultaneous read from and write to an i2c device
    pub fn transaction(&mut self, sent: &[u8], received: &mut [u8]) -> Result<(), I2cError> {
        let status = unsafe {
            HAL_TransactionI2C(self.port as i32, self.device_address, sent.as_ptr() as *mut u8, sent.len() as i32,
                                                                      received.as_mut_ptr(), received.len() as i32)
        };
        match status {
            -1 => Err(I2cError::TransferAbort),
            x if x >= 0 => Ok(()),
            _ => Err(I2cError::IOError),
        }
    }

    /// Read received message to inputed byte slice
    pub fn read(&self, received: &mut [u8]) -> Result<(), I2cError> {
        let status = unsafe {
            HAL_ReadI2C(self.port as i32, self.device_address, received.as_mut_ptr(), received.len() as i32)
        };
        match status {
            -1 => Err(I2cError::TransferAbort),
            x if x >= 0 => Ok(()),
            _ => Err(I2cError::IOError),
        }
    }

    /// Write byte slice to i2c device
    pub fn write(&mut self, sent: &[u8]) -> Result<(), I2cError> {
        let status = unsafe {
            HAL_WriteI2C(self.port as i32, self.device_address, sent.as_ptr() as *mut u8, sent.len() as i32)
        };
        match status {
            -1 => Err(I2cError::TransferAbort),
            x if x >= 0 => Ok(()),
            _ => Err(I2cError::IOError),
        }
    }

    /// Perform string transaction with connected i2c device
    pub fn string_transaction(&mut self, sent_string: &str, received_string: &mut str) -> Result<(), I2cError> {
        let status = unsafe {
            HAL_TransactionI2C(self.port as i32, self.device_address, sent_string.as_bytes().as_ptr() as *mut u8, sent_string.len() as i32,
                                                                      received_string.as_bytes().as_ptr() as *mut u8, received_string.len() as i32)
        };
        match status {
            -1 => Err(I2cError::TransferAbort),
            x if x >= 0 => Ok(()),
            _ => Err(I2cError::IOError),
        }
    }

    /// Read received message to str
    pub fn read_string(&self, received_string: &mut str) -> Result<(), I2cError> {
        let status = unsafe {
            HAL_ReadI2C(self.port as i32, self.device_address, received_string.as_bytes().as_ptr() as *mut u8, received_string.len() as i32)
        };
        match status {
            -1 => Err(I2cError::TransferAbort),
            x if x >= 0 => match received_string.len() == x as usize {
                true => Ok(()),
                false => Err(I2cError::InvalidReceiveString),
            },
            _ => Err(I2cError::IOError),
        }
    }

    /// Write str to i2c device
    pub fn write_string(&mut self, sent_string: &str) -> Result<(), I2cError> {
        let status = unsafe {
            HAL_WriteI2C(self.port as i32, self.device_address, sent_string.as_bytes().as_ptr() as *mut u8, sent_string.len() as i32)
        };
        match status {
            -1 => Err(I2cError::TransferAbort),
            x if x >= 0 => Ok(()),
            _ => Err(I2cError::IOError),
        }
    }
}

impl Drop for I2cInterface {
    /// Close I2C connection cleanly
    fn drop(&mut self) {
        unsafe { HAL_CloseI2C(self.port as i32); }
    }
}
