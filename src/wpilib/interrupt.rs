use wpilib::DigitalInput;
use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use std::sync::Arc;
use std::os::raw;
use std::mem;

pub enum WaitResult {
    Timeout = 0x0,
    RisingEdge = 0x1,
    FallingEdge = 0x100,
    Both = 0x101,
}

pub struct Interrupt {
    input: Arc<DigitalInput>,
    interrupt: HAL_InterruptHandle,
}

impl Interrupt {
    pub fn from_input(input: Arc<DigitalInput>) -> HalResult<Interrupt> {
        let handle = Interrupt::allocate_interrupt(true)?;
        Ok(Interrupt {
            input: input,
            interrupt: handle,
        })
    }

    pub fn from_channel(channel: i32) -> HalResult<Interrupt> {
        let input = Arc::new(DigitalInput::new(channel)?);
        Interrupt::from_input(input)
    }

    fn allocate_interrupt(watcher: bool) -> HalResult<HAL_InterruptHandle> {
        hal_call!(HAL_InitializeInterrupts(watcher as i32))
    }

    fn setup_source_edge(&mut self, rising: bool, falling: bool) -> HalResult<()> {
        hal_call!(HAL_SetInterruptUpSourceEdge(self.interrupt, rising as i32, falling as i32))
    }

    pub fn register_sync_interrupt(&mut self) -> HalResult<()> {
        hal_call!(HAL_RequestInterrupts(self.interrupt,
                                        self.input.get_handle(),
                                        mem::transmute(0)))?;
        self.setup_source_edge(true, false)
    }

    pub fn register_async_interrupt(&mut self,
                                    handler: unsafe extern "C" fn(u32, *mut raw::c_void),
                                    param: *mut raw::c_void)
                                    -> HalResult<()> {
        hal_call!(HAL_RequestInterrupts(self.interrupt,
                                        self.input.get_handle(),
                                        mem::transmute(0)))?;
        self.setup_source_edge(true, false)?;
        hal_call!(HAL_AttachInterruptHandler(self.interrupt, Some(handler), param))
    }

    pub unsafe fn wait(&mut self, timeout: f64, ignore_previous: bool) -> HalResult<WaitResult> {
        #[allow(unused_unsafe)]
        match hal_call!(HAL_WaitForInterrupt(self.interrupt, timeout, ignore_previous as i32))? {
            0x0 => Ok(WaitResult::Timeout),
            0x1 => Ok(WaitResult::RisingEdge),
            0x100 => Ok(WaitResult::FallingEdge),
            0x101 => Ok(WaitResult::Both),
            _ => Err(0),
        }
    }
}

impl Drop for Interrupt {
    fn drop(&mut self) {
        let _ = hal_call!(HAL_CleanInterrupts(self.interrupt));
    }
}
