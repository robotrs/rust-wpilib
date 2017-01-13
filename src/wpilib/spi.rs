use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::usage::*;
use std::mem;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// Some SPI port on the RoboRIO
pub enum SpiPort {
    Onboard0 = 0,
    Onboard1 = 1,
    Onboard2 = 2,
    Onboard3 = 3,
    MXP = 4,
}

/// An interface to the SPI bus
pub struct SpiInterface {
    port: i32,
}

impl SpiInterface {
    /// Create a new SPI interface on the specified port, returning an error if the initialization
    /// fails.
    pub fn new(port: SpiPort) -> HalResult<SpiInterface> {
        let port = port as i32;
        hal_call!(HAL_InitializeSPI(port))?;

        report_usage(ResourceType::SPI, port);

        Ok(SpiInterface { port: port })
    }

    /// Do a transaction on the SPI interface. `send` and `receive` must have the same length or
    /// the transaction will fail. Return the number of bytes read, or `None` if the transaction
    /// failed.
    pub fn transaction(&mut self, send: &[u8], receive: &mut [u8]) -> Option<i32> {
        if send.len() != receive.len() {
            return None;
        }

        let result = unsafe {
            HAL_TransactionSPI(self.port,
                               mem::transmute(&send[0] as *const u8),
                               &mut receive[0] as *mut u8,
                               send.len() as i32)
        };
        match result {
            -1 => None,
            b => Some(b),
        }
    }

    /// Write to the SPI interface and return the number of bytes written, or `None` for a failed
    /// write.
    pub fn write(&mut self, data: &[u8]) -> Option<i32> {
        let result = unsafe {
            HAL_WriteSPI(self.port,
                         mem::transmute(&data[0] as *const u8),
                         data.len() as i32)
        };
        match result {
            -1 => None,
            b => Some(b),
        }
    }

    /// Initate a read sequence by putting a 0 in the transmit buffer and doing a transfer.
    pub fn initiate_read(&mut self, buffer: &mut [u8]) -> Option<i32> {
        let send = vec![0; buffer.len()];
        self.transaction(&send, buffer)
    }

    /// Returns the number of bytes read from the receive queue (filled as a response to a previous
    /// write), or waits for the current transfer to complete if the queue is empty. If you want to
    /// initiate reading yourself, you probably want initiate_read.
    pub fn read(&self, data: &mut [u8]) -> Option<i32> {
        let result = unsafe { HAL_ReadSPI(self.port, &mut data[0] as *mut u8, data.len() as i32) };
        match result {
            -1 => None,
            b => Some(b),
        }
    }

    /// Set the SPI clock rate, up to 4MHz
    pub fn set_clock_rate(&mut self, rate: i32) {
        unsafe {
            HAL_SetSPISpeed(self.port, rate);
        }
    }
}

impl Drop for SpiInterface {
    fn drop(&mut self) {
        unsafe {
            HAL_CloseSPI(self.port);
        }
    }
}
