/// A generic interface to a speed controller with support for inverting the motor in case of a
/// backwards motor.
pub trait SpeedController {
    /// Set the speed for the speed controller to run at, from [-1, 1].
    fn set(&mut self, speed: f64);
    /// Get the previously set speed.
    fn get(&self) -> f64;

    /// Set whether or not this speed controller should run backwards.
    fn invert(&mut self, inverted: bool);
    /// Get the previously set inversion value.
    fn is_inverted(&self) -> bool;

    /// Temporarily disable the speed controller.
    fn disable(&mut self);
}
