use athena::wpilib_hal::*;
use athena::hal_call::*;
use athena::Throttler;

use std::{thread, time, mem, ffi, sync};

use atom::Atom;

const MAX_JOYSTICK_PORTS: usize = 6;
const MAX_JOYSTICK_AXES: usize = 12;
const MAX_JOYSTICK_POVS: usize = 12;

#[derive(Default)]
struct Joysticks {
    axes: [HAL_JoystickAxes; MAX_JOYSTICK_PORTS],
    povs: [HAL_JoystickPOVs; MAX_JOYSTICK_PORTS],
    buttons: [HAL_JoystickButtons; MAX_JOYSTICK_PORTS],
    descriptor: [HAL_JoystickDescriptor; MAX_JOYSTICK_PORTS],
}

#[derive(Debug)]
/// The robot's state
pub enum RobotState {
    Disabled,
    Autonomous,
    Teleop,
    Test,
    EStop,
}

type DSBuffer = Box<(Joysticks, HAL_ControlWord)>;

/// An interface to the driver station, FMS, and joysticks
pub struct DriverStation {
    data: sync::Arc<Atom<DSBuffer>>,
    joysticks: Joysticks,

    /// The state that the robot is currently in.
    pub state: RobotState,

    /// Whether or not the robot is attached to the FMS.
    pub fms_attached: bool,

    /// Whether or not the robot has connection to the driver station.
    pub ds_attached: bool,

    report_throttler: Throttler<f64>,

    waiter: sync::Arc<(sync::Mutex<bool>, sync::Condvar)>,

    join: Option<thread::JoinHandle<()>>,
}

static CREATE_DS: sync::Once = sync::ONCE_INIT;
static mut DRIVER_STATION: *mut DriverStation = 0 as *mut DriverStation;

#[derive(Debug, Copy, Clone)]
/// Some error involving joysticks
pub enum JoystickError {
    JoystickDNE,
    ChannelUnplugged,
    ChannelDNE,
}

#[derive(Debug, Copy, Clone)]
/// An alliance, red or blue
pub enum AllianceId {
    Red,
    Blue,
    Invalid,
}

impl DriverStation {
    fn new() -> DriverStation {
        let data_atom = sync::Arc::new(Atom::empty());
        let waiter = sync::Arc::new((sync::Mutex::new(false), sync::Condvar::new()));

        let mut ds = DriverStation {
            data: data_atom,
            joysticks: Joysticks::default(),
            state: RobotState::Disabled,
            fms_attached: false,
            ds_attached: false,

            // For now use an interval of 0 so we don't actually throttle messages, as the FPGA
            // timer isn't implemented yet.
            report_throttler: Throttler::new(0.0, 0.0),

            waiter: waiter,

            join: None,
        };

        ds.spawn_updater();
        ds
    }

    /// Spawn the updater thread. This should not be called after the constructor is finished.
    fn spawn_updater(&mut self) {
        let data_atom = self.data.clone();
        let waiter = self.waiter.clone();

        self.join = Some(thread::spawn(move || {
            loop {
                // Wait for the HAL to get new data
                unsafe {
                    HAL_WaitForDSData();
                }

                // Update the joysticks and control word using the new data.
                let mut joysticks = Joysticks::default();
                for stick in 0..MAX_JOYSTICK_PORTS {
                    unsafe {
                        HAL_GetJoystickAxes(stick as i32,
                                            &mut joysticks.axes[stick] as *mut HAL_JoystickAxes);
                        HAL_GetJoystickPOVs(stick as i32,
                                            &mut joysticks.povs[stick] as *mut HAL_JoystickPOVs);
                        HAL_GetJoystickButtons(stick as i32,
                                                    &mut joysticks.buttons[stick] as
                                                    *mut HAL_JoystickButtons);
                        HAL_GetJoystickDescriptor(stick as i32,
                                                       &mut joysticks.descriptor[stick] as
                                                       *mut HAL_JoystickDescriptor);
                    }
                }

                let mut control_word: HAL_ControlWord = HAL_ControlWord::default();
                unsafe {
                    HAL_GetControlWord(&mut control_word as *mut HAL_ControlWord);
                }

                // Write that data into the atom for usage by callers
                data_atom.swap(Box::new((joysticks, control_word)));

                // Notify any threads waiting for data
                {
                    let mut guard = waiter.0.lock().unwrap();
                    *guard = true;
                    waiter.1.notify_all();
                }
            }
        }));
    }

    /// Read new joystick and control word data
    fn update_data(&mut self) {
        if let Some(boxed_data) = self.data.take() {
            let new_control_word = boxed_data.1;
            self.joysticks = boxed_data.0;
            self.state = if new_control_word.enabled() {
                if new_control_word.autonomous() {
                    RobotState::Autonomous
                } else {
                    RobotState::Teleop
                }
            } else if new_control_word.eStop() {
                RobotState::EStop
            } else {
                RobotState::Disabled
            };
            self.fms_attached = new_control_word.fmsAttached();
            self.ds_attached = new_control_word.dsAttached();
        }
    }

    /// Report an error to the driver station in its most general form. Don't use this directly,
    /// instead use it in other error reporting methods.
    fn report(&self, is_error: bool, code: i32, error: &str, location: &str, stack: &str) {
        unsafe {
            HAL_SendError(is_error as i32,
                          code,
                          false as i32,
                          ffi::CString::new(error).unwrap().into_raw(),
                          ffi::CString::new(location).unwrap().into_raw(),
                          ffi::CString::new(stack).unwrap().into_raw(),
                          true as i32);
        }
    }

    fn report_error(&mut self, error: &str) {
        self.report(true, 1, error, "", "");
    }

    fn report_warning(&mut self, warning: &str) {
        self.report(false, 1, warning, "", "");
    }

    /// Report a message at a throttled rate
    fn report_throttled(&mut self, is_error: bool, message: &str) {
        // Don't actually throttle it; FPGA timer is unimplemented
        let now = 1f64;
        if self.report_throttler.update(now) {
            self.report(is_error, 1, message, "", "");
        }
    }

    /// Get an instance of the driver station. This will create a new instance if one does not
    /// exist.
    pub fn instance() -> &'static mut DriverStation {
        unsafe {
            CREATE_DS.call_once(|| {
                DRIVER_STATION = mem::transmute(Box::new(DriverStation::new()));
            });
            &mut *DRIVER_STATION
        }
    }

    /// Get an axis on a joystick, in the range of [-1, 1].
    pub fn get_joystick_axis(&mut self, stick: usize, axis: usize) -> Result<f32, JoystickError> {
        self.update_data();

        if stick >= MAX_JOYSTICK_PORTS {
            self.report_throttled(true, "Bad joystick");
            Err(JoystickError::JoystickDNE)
        } else if axis >= MAX_JOYSTICK_AXES {
            self.report_throttled(true, "Bad joystick axis");
            Err(JoystickError::ChannelDNE)
        } else if axis >= self.joysticks.axes[stick].count as usize {
            self.report_throttled(true,
                                  "Joystick axis missing, check if all controllers are plugged in");
            Err(JoystickError::ChannelUnplugged)
        } else {
            Ok(self.joysticks.axes[stick].axes[axis])
        }
    }

    /// Get the position of a POV switch, in degrees.
    pub fn get_joystick_pov(&mut self, stick: usize, pov: usize) -> Result<i16, JoystickError> {
        self.update_data();

        if stick >= MAX_JOYSTICK_POVS {
            self.report_throttled(true, "Bad joystick");
            Err(JoystickError::JoystickDNE)
        } else if pov >= MAX_JOYSTICK_AXES {
            self.report_throttled(true, "Bad joystick pov");
            Err(JoystickError::ChannelDNE)
        } else if pov >= self.joysticks.povs[stick].count as usize {
            self.report_throttled(true,
                                  "Joystick pov missing, check if all controllers are plugged in");
            Err(JoystickError::ChannelUnplugged)
        } else {
            Ok(self.joysticks.povs[stick].povs[pov])
        }
    }

    /// Get the state of a button on a joystick.
    pub fn get_joystick_button(&mut self,
                               stick: usize,
                               button: usize)
                               -> Result<bool, JoystickError> {
        self.update_data();

        if stick >= MAX_JOYSTICK_POVS {
            self.report_throttled(true, "Bad joystick");
            Err(JoystickError::JoystickDNE)
        } else if button == 0 {
            self.report_throttled(true, "Bad joystick button (button IDs start from 1)");
            Err(JoystickError::ChannelDNE)
        } else if button >= self.joysticks.povs[stick].count as usize {
            self.report_throttled(true,
                                  "Joystick button missing, check if all controllers are plugged \
                                   in");
            Err(JoystickError::ChannelUnplugged)
        } else {
            let mask = 1 << (button - 1);
            Ok(self.joysticks.buttons[stick].buttons & mask != 0)
        }
    }

    /// Get the alliance the robot is on.
    pub fn get_alliance(&self) -> HalResult<AllianceId> {
        match hal_call!(HAL_GetAllianceStation())? {
            HAL_AllianceStationID::HAL_AllianceStationID_kRed1 |
            HAL_AllianceStationID::HAL_AllianceStationID_kRed2 |
            HAL_AllianceStationID::HAL_AllianceStationID_kRed3 => Ok(AllianceId::Red),
            HAL_AllianceStationID::HAL_AllianceStationID_kBlue1 |
            HAL_AllianceStationID::HAL_AllianceStationID_kBlue2 |
            HAL_AllianceStationID::HAL_AllianceStationID_kBlue3 => Ok(AllianceId::Blue),
        }
    }

    /// Get the id for the station the driver station is at, as an integer.
    pub fn get_station(&self) -> HalResult<i32> {
        match hal_call!(HAL_GetAllianceStation())? {
            HAL_AllianceStationID::HAL_AllianceStationID_kRed1 |
            HAL_AllianceStationID::HAL_AllianceStationID_kBlue1 => Ok(1),
            HAL_AllianceStationID::HAL_AllianceStationID_kRed2 |
            HAL_AllianceStationID::HAL_AllianceStationID_kBlue2 => Ok(2),
            HAL_AllianceStationID::HAL_AllianceStationID_kRed3 |
            HAL_AllianceStationID::HAL_AllianceStationID_kBlue3 => Ok(3),
        }
    }

    /// Waits for a new driver station packet
    pub fn wait_for_data(&self) {
        let &(ref wait_lock, ref wait_cond) = &*self.waiter;
        let mut has_data = wait_lock.lock().unwrap();
        while !*has_data {
            has_data = wait_cond.wait(has_data).unwrap();
        }
    }

    /// Waits for a new driver station packet and returns true, or returns false if timeout is
    /// exceeded.
    pub fn wait_for_data_or_timeout(&self, timeout: time::Duration) -> bool {
        let &(ref wait_lock, ref wait_cond) = &*self.waiter;
        let mut has_data = wait_lock.lock().unwrap();

        while !*has_data {
            let result = wait_cond.wait_timeout(has_data, timeout).unwrap();
            if result.1.timed_out() {
                return false;
            } else {
                has_data = result.0;
            }
        }
        true
    }
}

impl Drop for DriverStation {
    fn drop(&mut self) {
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}
