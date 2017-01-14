use wpilib::speed_controller::SpeedController;
use wpilib::pwm::*;
use wpilib::hal_call::*;
use wpilib::usage::*;

/// A PWM-based speed controller, like the VictorSP.
///
/// # Usage
/// ```
/// let mut victor = PwmSpeedController::victor_sp(0, false).unwrap();
/// ...
/// if (at_goal()) {
///     victor.disable();
/// } else {
///     victor.set(0.5);
/// }
/// ```
pub struct PwmSpeedController {
    pwm: Pwm,
    inverted: bool,
}

impl PwmSpeedController {
    /// Create a new VictorSP speed controller object with the correct parameters for the speed
    /// controller.
    pub fn victor_sp(channel: i32, inverted: bool) -> HalResult<PwmSpeedController> {
        let mut pwm = Pwm::new(channel)?;

        pwm.set_config(2.004, 1.52, 1.50, 1.48, 0.997)?;
        pwm.slow_period(PeriodMultiplier::k1X)?;
        pwm.set_speed(0f64)?;
        pwm.set_zero_latch()?;

        report_usage(ResourceType::VictorSP, channel);

        Ok(PwmSpeedController {
            pwm: pwm,
            inverted: inverted,
        })
    }

    /// Create a new Victor speed controller object with the correct parameters for the speed
    /// controller. Note: this creates an object for the Victor from the old control system. If you
    /// have a new VictorSP, use `PwmSpeedController::victor_sp`.
    pub fn victor(channel: i32, inverted: bool) -> HalResult<PwmSpeedController> {
        let mut pwm = Pwm::new(channel)?;

        pwm.set_config(2.027, 1.525, 1.507, 1.49, 1.026)?;
        pwm.slow_period(PeriodMultiplier::k1X)?;
        pwm.set_speed(0f64)?;
        pwm.set_zero_latch()?;

        report_usage(ResourceType::Victor, channel);

        Ok(PwmSpeedController {
            pwm: pwm,
            inverted: inverted,
        })
    }

    /// Create a new Talon or Talon SR speed controller object with the correct parameters for the
    /// speed controller. Note: if you have a new Talon SRX, use `PwmSpeedController::talon_srx`.
    pub fn talon(channel: i32, inverted: bool) -> HalResult<PwmSpeedController> {
        let mut pwm = Pwm::new(channel)?;

        pwm.set_config(2.037, 1.539, 1.513, 1.87, 0.989)?;
        pwm.slow_period(PeriodMultiplier::k1X)?;
        pwm.set_speed(0f64)?;
        pwm.set_zero_latch()?;

        report_usage(ResourceType::Talon, channel);

        Ok(PwmSpeedController {
            pwm: pwm,
            inverted: inverted,
        })
    }

    /// Create a new Talon SRX speed controller object with the correct parameters for this speed
    /// controller. This is the PWM constructor - CAN support is not yet implemented (Coming soon!)
    pub fn talon_srx(channel: i32, inverted: bool) -> HalResult<PwmSpeedController> {
        let mut pwm = Pwm::new(channel)?;

        pwm.set_config(2.004, 1.52, 1.50, 1.48, 0.997)?;
        pwm.slow_period(PeriodMultiplier::k1X)?;
        pwm.set_speed(0f64)?;
        pwm.set_zero_latch()?;

        report_usage(ResourceType::TalonSRX, channel);

        Ok(PwmSpeedController {
            pwm: pwm,
            inverted: inverted,
        })
    }
}

impl SpeedController for PwmSpeedController {
    fn set(&mut self, speed: f64) {
        let speed = if self.is_inverted() { -speed } else { speed };
        self.pwm.set_speed(speed).unwrap();
    }

    fn get(&self) -> f64 {
        self.pwm.get_speed().unwrap()
    }

    fn invert(&mut self, inverted: bool) {
        self.inverted = inverted;
    }

    fn is_inverted(&self) -> bool {
        self.inverted
    }

    fn disable(&mut self) {
        self.pwm.disable().unwrap();
    }
}
