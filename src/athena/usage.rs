#![allow(missing_docs)]

use athena::wpilib_hal::*;
use std::ptr;
use std::os::raw;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ResourceType {
    Controller = 0,
    Module = 1,
    Language = 2,
    CANPlugin = 3,
    Accelerometer = 4,
    ADXL345 = 5,
    AnalogChannel = 6,
    AnalogTrigger = 7,
    AnalogTriggerOutput = 8,
    CANJaguar = 9,
    Compressor = 10,
    Counter = 11,
    Dashboard = 12,
    DigitalInput = 13,
    DigitalOutput = 14,
    DriverStationCIO = 15,
    DriverStationEIO = 16,
    DriverStationLCD = 17,
    Encoder = 18,
    GearTooth = 19,
    Gyro = 20,
    I2C = 21,
    Framework = 22,
    Jaguar = 23,
    Joystick = 24,
    Kinect = 25,
    KinectStick = 26,
    PIDController = 27,
    Preferences = 28,
    PWM = 29,
    Relay = 30,
    RobotDrive = 31,
    SerialPort = 32,
    Servo = 33,
    Solenoid = 34,
    SPI = 35,
    Task = 36,
    Ultrasonic = 37,
    Victor = 38,
    Button = 39,
    Command = 40,
    AxisCamera = 41,
    PCVideoServer = 42,
    SmartDashboard = 43,
    Talon = 44,
    HiTechnicColorSensor = 45,
    HiTechnicAccel = 46,
    HiTechnicCompass = 47,
    SRF08 = 48,
    AnalogOutput = 49,
    VictorSP = 50,
    TalonSRX = 51,
    CANTalonSRX = 52,
    ADXL362 = 53,
    ADXRS450 = 54,
    RevSPARK = 55,
    MindsensorsSD540 = 56,
    DigitalFilter = 57,
    ADIS16448 = 58,
    PDP = 59,
    PCM = 60,
    PigeonIMU = 61,
}

pub const LANGUAGE_RUST: i32 = 6;

/// Report the usage of a specific resource type with an `instance` value attached.
pub fn report_usage(resource: ResourceType, instance: i32) {
    unsafe {
        HAL_Report(resource as i32, instance, 0, ptr::null());
    }
}

/// Just a safe wrapper around HAL_Report
pub fn report_usage_extras(resource: ResourceType,
                           instance: i32,
                           context: i32,
                           feature: *const raw::c_char) {
    unsafe {
        HAL_Report(resource as i32, instance, context, feature);
    }
}
