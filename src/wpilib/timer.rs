use wpilib::fpga;

/// Gets the FPGA time in seconds since the FPGA reset
pub fn get_time_seconds() -> f64 {
    fpga::get_time_us() as f64 * 1e-6
}

pub struct Timer {
    start_time: f64,
    accumulated_time: f64,
    running: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: 0f64,
            accumulated_time: 0f64,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.start_time = get_time_seconds();
        self.running = true;
        self.accumulated_time = 0f64;
    }

    pub fn get(&self) -> f64 {
        if self.running {
            get_time_seconds() - self.start_time
        } else {
            self.accumulated_time
        }
    }

    pub fn stop(&mut self) {
        self.accumulated_time = self.get();
        self.running = false;
        self.start_time = 0f64;
    }

    pub fn reset(&mut self) {
        self.start_time = get_time_seconds();
        self.accumulated_time = 0f64;
    }

    pub fn has_period_passed(&self, time: f64) -> bool {
        self.get() > time
    }
}
