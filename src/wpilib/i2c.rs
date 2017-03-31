use wpilib::wpilib_hal::*;

/// Helper enum that describes one of two possible locations on the roborio for i2c devices
#[derive(Copy, Clone)]
pub enum I2cPort {
    /// Enum option for i2c device plugged into the onboard i2c connectors on the roborio
    OnBoard = 0,
    /// Enum option for i2c device, such as a navX board, connected through the MXP port
    MXP = 1,
}

/// Describes various errored or otherwise special cases that may result from an i2c transaction
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
    /// users create a new I2cInterface here
    pub fn new(p: I2cPort, addr: i32, init_status: &mut i32) -> I2cInterface {
        unsafe { HAL_InitializeI2C(p as i32, init_status); };
        I2cInterface { port: p, device_address: addr }
    }

    /// perform a simultaneous read from and write to an i2c device
    pub fn transaction(&mut self, sent: &mut [u8], received: &mut [u8]) -> Result<(), I2cError> {
        let status = unsafe {
            HAL_TransactionI2C(self.port as i32, self.device_address, sent.as_mut_ptr(), sent.len() as i32,
                                                                      received.as_mut_ptr(), received.len() as i32)
        };
        match status >= 0 {
            true => Ok(()),
            false => match status {
                -1 => Err(I2cError::TransferAbort),
                _ => Err(I2cError::IOError),
            }
        }
    }

    /// reads received message to inputed byte slice
    pub fn read(&self, received: &mut [u8]) -> Result<(), I2cError> {
        let status = unsafe {
            HAL_ReadI2C(self.port as i32, self.device_address, received.as_mut_ptr(), received.len() as i32)
        };
        match status >= 0 {
            true => Ok(()),
            false => match status {
                -1 => Err(I2cError::TransferAbort),
                _ => Err(I2cError::IOError),
            }
        }
    }

    /// writes byte slice to i2c device
    pub fn write(&mut self, sent: &mut [u8]) -> Result<(), I2cError> {
        let status = unsafe {
            HAL_WriteI2C(self.port as i32, self.device_address, sent.as_mut_ptr(), sent.len() as i32)
        };
        match status >= 0 {
            true => Ok(()),
            false => match status {
                -1 => Err(I2cError::TransferAbort),
                _ => Err(I2cError::IOError),
            }
        }
    }
}

impl Drop for I2cInterface {
    /// destructor closes I2C connection cleanly
    fn drop(&mut self) {
        unsafe { HAL_CloseI2C(self.port as i32); }
    }
}
