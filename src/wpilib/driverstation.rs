use wpilib::wpilib_hal as hal;
use std::thread;
use std::sync::mpsc;
use std::ptr;
use std::mem::transmute;

const MAX_JOYSTICK_PORTS: usize = 6;
const MAX_JOYSTICK_AXES: usize = 12;
const MAX_JOYSTICK_POVS: usize = 12;

#[derive(Default)]
struct Joysticks {
    axes: [hal::HAL_JoystickAxes; MAX_JOYSTICK_PORTS],
    povs: [hal::HAL_JoystickPOVs; MAX_JOYSTICK_PORTS],
    buttons: [hal::HAL_JoystickButtons; MAX_JOYSTICK_PORTS],
    descriptor: [hal::HAL_JoystickDescriptor; MAX_JOYSTICK_PORTS],
}

#[derive(Debug)]
pub enum RobotState {
    Disabled,
    Autonomous,
    Teleop,
    Test,
    EStop,
}

type DSChannelData = (Joysticks, hal::HAL_ControlWord);

pub struct DriverStation {
    data_channel: mpsc::Receiver<DSChannelData>,
    joysticks: Joysticks,
    pub state: RobotState,
    pub fms_attached: bool,
    pub ds_attached: bool,
}

static mut driver_station: *mut DriverStation = 0 as *mut DriverStation;

#[derive(Debug)]
pub enum JoystickError {
    JoystickDNE,
    ChannelUnplugged,
    ChannelDNE,
}

impl DriverStation {
    fn new() -> DriverStation {
        let (tx, rx) = mpsc::channel::<DSChannelData>();
        let join = thread::spawn(move || {
            loop {
                let mut joysticks = Joysticks::default();
                for stick in 0..MAX_JOYSTICK_PORTS {
                    unsafe {
                        hal::HAL_GetJoystickAxes(stick as i32,
                                                 &mut joysticks.axes[stick] as
                                                 *mut hal::HAL_JoystickAxes);
                        hal::HAL_GetJoystickPOVs(stick as i32,
                                                 &mut joysticks.povs[stick] as
                                                 *mut hal::HAL_JoystickPOVs);
                        hal::HAL_GetJoystickButtons(stick as i32,
                                                    &mut joysticks.buttons[stick] as
                                                    *mut hal::HAL_JoystickButtons);
                        hal::HAL_GetJoystickDescriptor(stick as i32,
                                                       &mut joysticks.descriptor[stick] as
                                                       *mut hal::HAL_JoystickDescriptor);
                    }
                }

                let mut control_word: hal::HAL_ControlWord = hal::HAL_ControlWord::default();
                unsafe {
                    hal::HAL_GetControlWord(&mut control_word as *mut hal::HAL_ControlWord);
                }
                tx.send((joysticks, control_word));
            }
        });
        DriverStation {
            data_channel: rx,
            joysticks: Joysticks::default(),
            state: RobotState::Disabled,
            fms_attached: false,
            ds_attached: false,
        }
    }

    fn update_data(&mut self) {
        if let Ok((new_joysticks, new_control_word)) = self.data_channel.try_recv() {
            self.joysticks = new_joysticks;
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

    pub fn instance() -> &'static mut DriverStation {
        unsafe {
            if driver_station == 0 as *mut DriverStation {
                driver_station = transmute(Box::new(DriverStation::new()));
            }
            &mut *driver_station
        }
    }

    pub fn get_joystick_axis(&mut self, stick: usize, axis: usize) -> Result<f32, JoystickError> {
        self.update_data();

        if stick >= MAX_JOYSTICK_PORTS {
            Err(JoystickError::JoystickDNE)
        } else if axis >= MAX_JOYSTICK_AXES {
            Err(JoystickError::ChannelDNE)
        } else if axis >= self.joysticks.axes[stick].count as usize {
            Err(JoystickError::ChannelUnplugged)
        } else {
            Ok(self.joysticks.axes[stick].axes[axis])
        }
    }

    pub fn get_joystick_pov(&mut self, stick: usize, pov: usize) -> Result<i16, JoystickError> {
        self.update_data();

        if stick >= MAX_JOYSTICK_POVS {
            Err(JoystickError::JoystickDNE)
        } else if pov >= MAX_JOYSTICK_AXES {
            Err(JoystickError::ChannelDNE)
        } else if pov >= self.joysticks.povs[stick].count as usize {
            Err(JoystickError::ChannelUnplugged)
        } else {
            Ok(self.joysticks.povs[stick].povs[pov])
        }
    }

    pub fn get_joystick_button(&mut self,
                               stick: usize,
                               button: usize)
                               -> Result<bool, JoystickError> {
        self.update_data();

        if stick >= MAX_JOYSTICK_POVS {
            Err(JoystickError::JoystickDNE)
        } else if button == 0 {
            Err(JoystickError::ChannelDNE)
        } else if button >= self.joysticks.povs[stick].count as usize {
            Err(JoystickError::ChannelUnplugged)
        } else {
            let mask = 1 << (button - 1);
            Ok(self.joysticks.buttons[stick].buttons & mask != 0)
        }
    }
}
