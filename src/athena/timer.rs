use athena::fpga;

/// Gets the FPGA time in seconds since the FPGA reset
pub fn get_time_seconds() -> f64 {
    fpga::get_time_us() as f64 * 1e-6
}

/// A timer that can be paused and reset
pub struct Timer {
    start_time: f64,
    accumulated_time: f64,
    running: bool,
}

impl Timer {
    /// Create a new timer with zero time on the clock. The timer will not start running until
    /// start() is called.
    pub fn new() -> Timer {
        Timer {
            start_time: 0f64,
            accumulated_time: 0f64,
            running: false,
        }
    }

    /// Create a new timer that will immediately begin running.
    pub fn new_running() -> Timer {
        Timer {
            start_time: get_time_seconds(),
            accumulated_time: 0f64,
            running: true,
        }
    }

    /// Start the timer.
    pub fn start(&mut self) {
        self.start_time = get_time_seconds();
        self.running = true;
        self.accumulated_time = 0f64;
    }

    /// Get the elapsed time in seconds.
    pub fn get(&self) -> f64 {
        if self.running {
            get_time_seconds() - self.start_time
        } else {
            self.accumulated_time
        }
    }

    /// Stop the clock and freeze the elapsed time.
    pub fn stop(&mut self) {
        self.accumulated_time = self.get();
        self.running = false;
        self.start_time = 0f64;
    }

    /// Reset the elapsed time to zero.
    pub fn reset(&mut self) {
        self.start_time = get_time_seconds();
        self.accumulated_time = 0f64;
    }

    /// Has some period of time passed on the clock?
    pub fn has_period_passed(&self, time: f64) -> bool {
        self.get() > time
    }
}
