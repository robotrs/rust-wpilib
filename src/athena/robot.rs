use athena::wpilib_hal::*;

/// The base class from which all robots should be derived.
///
/// # Usage
///
/// ```
/// struct TestRobot {};
///
/// impl Robot for TestRobot {
///     fn new() -> TestRobot {
///         TestRobot{}
///     }
///
///     fn run(self) {
///         // Do something...
///     }
/// }
///
/// fn main() {
///     TestRobot::main();
/// }
/// ```
pub trait Robot: Sized {
    /// Run the robot class. This will be called once, at the beginning of the program, after
    /// initialization.
    fn run(self);

    /// Create an instance of the robot class.
    fn new() -> Self;

    /// Run the robot statically.
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
