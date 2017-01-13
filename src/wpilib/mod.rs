#[macro_use]
mod robot;
pub use self::robot::*;

#[macro_use]
mod hal_call;

mod wpilib_hal;

mod driverstation;
pub use self::driverstation::DriverStation;

mod throttler;
pub use self::throttler::Throttler;

/// Useful FPGA functions
pub mod fpga;

mod timer;
pub use self::timer::*;

/// Interrupt wrapper functions
pub mod interrupts;

/// Sensor info funcitons
pub mod sensor;

mod digital_input;
pub use self::digital_input::DigitalInput;

mod digital_output;
pub use self::digital_output::DigitalOutput;

mod encoder;
pub use self::encoder::Encoder;

mod pdp;
pub use self::pdp::PowerDistributionPanel;

/// Functions for information about the robot's state that are not contained anywhere else.
pub mod robot_state;

mod analog_input;
pub use self::analog_input::AnalogInput;

mod spi;
pub use self::spi::SpiInterface;
