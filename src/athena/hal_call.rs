use athena::wpilib_hal::*;
use std::{ffi, fmt};

#[derive(Copy, Clone)]
pub struct HalError(pub i32);

impl From<i32> for HalError {
    fn from(code: i32) -> Self {
        HalError(code)
    }
}

impl fmt::Debug for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let error_string = unsafe { ffi::CStr::from_ptr(HAL_GetErrorMessage(self.0)) };
        write!(f, "HalError {{ {} }}", error_string.to_str().unwrap())
    }
}

pub type HalResult<T> = Result<T, HalError>;

// impl<T> HalError<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Ok(value) => value,
//             Err(code) => {
//                 let error_string = unsafe { ffi::CStr::from_ptr(HAL_GetErrorMessage(code)) };
//                 panic!("{}", error_string.as_str());
//             }
//         }
//     }
// }

#[macro_export]
macro_rules! hal_call {
    ($function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(HalError::from(status)) }
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(HalError::from(status)) }
    }};
}
