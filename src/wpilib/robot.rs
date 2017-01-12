use wpilib::wpilib_hal::*;

pub trait Robot: Sized {
    fn run(self);

    fn new() -> Self;

    fn main() {
        // Initialize HAL
        unsafe {
            let status = HAL_Initialize(0);
            if status != 1 {
                panic!("WPILib HAL failed to initialize!");
            }
        }
        let robot = Self::new();
        robot.run();
    }
}
