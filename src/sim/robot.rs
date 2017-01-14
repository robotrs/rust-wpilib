/// The base class from which all robots should be derived.
///
/// # Usage
///
/// ```
/// use wpilib::Robot;
/// struct TestRobot;
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
        let robot = Self::new();
        robot.run();
    }
}
