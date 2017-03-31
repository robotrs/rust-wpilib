use wpilib::hal_call::*;
use std::convert::From;

/// An error reading or writing to/from the CAN bus
#[derive(Debug, Copy, Clone)]
pub enum CanError {
    /// CAN frame has not been received within specified period of time.
    RxTimeout,
    /// Not used.
    TxTimeout,
    /// Caller passed an invalid param
    InvalidParamValue,
    /// Specified CAN Id is invalid.
    UnexpectedArbId,
    /// Could not transmit the CAN frame.
    TxFailed,
    /// Have not received an value response for signal.
    SigNotUpdated,
    /// Caller attempted to insert data into a buffer that is full.
    BufferFull,
    /// Some other error happened
    Other(HalError)
}

impl From<i32> for CanError {
    fn from(value: i32) -> CanError {
        match value {
            1 => CanError::RxTimeout,
            2 => CanError::InvalidParamValue,
            3 => CanError::UnexpectedArbId,
            4 => CanError::TxFailed,
            5 => CanError::SigNotUpdated,
            6 => CanError::BufferFull,
            e => CanError::Other(HalError(e)),
        }
    }
}

impl From<HalError> for CanError {
    fn from(value: HalError) -> CanError {
        let HalError(value) = value;
        From::from(value)
    }
}
