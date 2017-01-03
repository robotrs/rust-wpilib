use wpilib::wpilib_hal;

pub trait Robot: Sized {
    fn run(self);

    fn new() -> Self;
}

#[macro_export]
macro_rules! start_robot_class {
    ($robot_class:ident) => {
        fn main() {
            unsafe {
                let status = wpilib_hal::HAL_Initialize(0);
                if status != 0 {
                    panic!("WPILib HAL failed to initialize with error {}", status);
                }
                // Report the language as something other than the supported ones
                wpilib_hal::HAL_Report(2, 6, 0, std::ptr::null());
                let robot = $robot_class::new();
                robot.run();
            }
        }
    }
}
