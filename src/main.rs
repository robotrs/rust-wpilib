#[macro_use]
extern crate wpilib;
use wpilib::*;

struct TestRobot;

impl Robot for TestRobot {
    fn run(self) {
        println!("Running!");
        loop {
            println!("{:?}", DriverStation::instance().get_joystick_axis(0, 1));
            println!("{:?}", fpga::get_time_us());
        }
    }

    fn new() -> TestRobot {
        TestRobot {}
    }
}

fn main() {
    TestRobot::main();
}
