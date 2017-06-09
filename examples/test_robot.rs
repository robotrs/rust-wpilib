extern crate wpilib;
use wpilib::*;

struct TestRobot;

impl Robot for TestRobot {
    fn run(self) {
        println!("Running!");
        let mut stick = Joystick::new(0);
        let pcm = BufferedPcm::new(0).unwrap();
        let mut solenoid = pcm.make_solenoid(0).unwrap();
        loop {
            println!("{:?}", stick.get_raw_axis(1));
            println!("{:?}", fpga::get_time_us());
            solenoid.set(true);
            pcm.flush().unwrap();
        }
    }

    fn new() -> TestRobot {
        TestRobot {}
    }
}

fn main() {
    TestRobot::main();
}
