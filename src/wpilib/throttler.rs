// use std::ops::Add;

/// Handles only doing some task once per set interval.
pub struct Throttler<T, S = T>
    // where T: Add<S> + PartialOrd, <T as Add<S>>::Output = T
{
    next_send: T,
    interval: S,
}

// impl<T, S> Throttler<T, S>
    // where T: Add<S> + PartialOrd, <T as Add<S>>::Output = T
impl Throttler<f64, f64>
{
    /// Create a new throttler.
    pub fn new(now: f64, interval: f64) -> Throttler<f64, f64> {
        Throttler {
            next_send: now + interval,
            interval: interval,
        }
    }

    /// Update the throttler. Returns true if the task should be performed.
    pub fn update(&mut self, now: f64) -> bool {
        if now > self.next_send {
            self.next_send = self.next_send + self.interval;
            true
        } else {
            false
        }
    }
}
