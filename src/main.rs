#[macro_use]
extern crate wpilib;
use wpilib::*;

struct TestRobot;

impl Robot for TestRobot {
    fn run(self) {
        println!("Running!");
    }

    fn new() -> TestRobot {
        TestRobot {}
    }
}

#[cfg(not(test))]
fn main() {
    TestRobot::main();
}
