pub type HalResult<T> = Result<T, i32>;

#[macro_export]
macro_rules! hal_call {
    ($function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(status) }
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(status) }
    }};
}
