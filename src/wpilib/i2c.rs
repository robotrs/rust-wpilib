use wpilib::wpilib_hal::*;

use std::slice;

/// Helper enum that describes one of two possible locations on the roborio for i2c devices
#[derive(Copy, Clone)]
pub enum I2cPort {
    /// Enum option for i2c device plugged into the onboard i2c connectors on the roborio
    OnBoard = 0,
    /// Enum option for i2c device, such as a navX board, connected through the MXP port
    MXP = 1,
}

/// Describes various errored or otherwise special cases that may result from an i2c transaction
pub enum I2cState {
    // TODO find is there is any way to get more i2c transaction information out of HAl than just
    // success or failure

    /// Indicates an error in an i2c transaction
    IOError,
    /// Indicates when no string, or an invalid string was received from the i2c transaction
    InvalidReceiveString,
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

    /// base function for a single i2c transaction, in which both parties may send and receive data
    /// note: unlike wpilib proper, transaction returns a new String as the result, and consumes the str parameter
    pub fn transaction(&self, sent: &str, send_size: i32, received: &str, receive_size: i32) -> Result<String, I2cState> {
        let send_string = String::from(sent);
        let receive_string = String::from(received);
        let send_bytes: *mut u8 = send_string.into_bytes().as_mut_ptr();
        let receive_bytes: *mut u8 = receive_string.into_bytes().as_mut_ptr();
        let status = unsafe {
            HAL_TransactionI2C(self.port as i32, self.device_address, send_bytes, send_size, receive_bytes, receive_size)
        };
        match status < 0 {
            true => {
                unsafe {
                    match String::from_utf8(slice::from_raw_parts(receive_bytes, receive_size as usize).to_vec()) {
                        Ok(msg) => Ok(msg),
                        _ => {
                            Err(I2cState::InvalidReceiveString)
                        },
                    }
                }
            },
            false => Err(I2cState::IOError),
        }
    }

    /// helper function to read from i2c using the size of the expected message
    /// returns a string with capacity of receive size
    pub fn read(&self, receive_size: i32) -> Result<String, I2cState> {
        let mut container: Vec<u8> = vec![];
        let mut i = 0;
        while i < receive_size {
            container.push(0u8);
            i += 1;
        }
        let buffer: *mut u8 = container.as_mut_ptr();
        let status = unsafe {
            HAL_ReadI2C(self.port as i32, self.device_address, buffer, receive_size)
        };
        match status < 0 {
            true => unsafe { Ok(String::from_raw_parts(buffer, receive_size as usize, receive_size as usize)) },
            _ => Err(I2cState::IOError),
        }
    }

    /// helper function to write to i2c
    /// to use, pass the message and its size
    /// note: analogous to WriteBulk on wpilib proper
    pub fn write(&self, sent: &str, send_size: i32) -> Result<bool, I2cState> {
        let send_string = String::from(sent);
        let send_bytes: *mut u8 = send_string.into_bytes().as_mut_ptr();
        let status = unsafe {
            HAL_WriteI2C(self.port as i32, self.device_address, send_bytes, send_size)
        };
        match status < 0 {
            true => Ok(true),
            _ => Err(I2cState::IOError),
        }
    }
}

impl Drop for I2cInterface {
    /// destructor closes I2C connection cleanly
    fn drop(&mut self) {
        unsafe { HAL_CloseI2C(self.port as i32); }
    }
}
