#[macro_use]
extern crate wpilib;
use wpilib::*;

struct TestRobot;

impl Robot for TestRobot {
    fn run(self) {
        println!("Running!");
        let stick = Joystick::new(0);
        loop {
            println!("{:?}", stick.get_raw_axis(1));
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
