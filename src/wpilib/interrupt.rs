use wpilib::DigitalInput;
use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use std::sync::Arc;
use std::os::raw;
use std::mem;


/// The result of an interrupt wait
pub enum WaitResult {
    /// Waiting on the interrupt timed out.
    Timeout = 0x0,
    /// The interrupt was triggered by a rising edge.
    RisingEdge = 0x1,
    /// The interrupt was triggered by a falling edge.
    FallingEdge = 0x100,
    /// The interrupt was triggered by both a rising edge and a falling edge.
    Both = 0x101,
}

/// The interface to a hardware interrupt. An interrupt allows the user to wait on a specific
/// hardware event - for example, for a DIO pin to go high - before executing any instructions. In
/// addition, it supports using C-style callback functions for asynchronous interrupts.
///
/// An interrupt can be created based on an existing digital input or by creating a new one.
pub struct Interrupt {
    input: Arc<DigitalInput>,
    interrupt: HAL_InterruptHandle,
}

impl Interrupt {
    /// Create an interrupt from an existing digital input, returning an error if initialization
    /// fails.
    pub fn from_input(input: Arc<DigitalInput>) -> HalResult<Interrupt> {
        let handle = Interrupt::allocate_interrupt(true)?;
        Ok(Interrupt {
            input: input,
            interrupt: handle,
        })
    }

    /// Create an interrupt on a new digital input, returning an error if initialization fails.
    pub fn from_channel(channel: i32) -> HalResult<Interrupt> {
        let input = Arc::new(DigitalInput::new(channel)?);
        Interrupt::from_input(input)
    }

    /// Allocate a new interrupt.
    fn allocate_interrupt(watcher: bool) -> HalResult<HAL_InterruptHandle> {
        hal_call!(HAL_InitializeInterrupts(watcher as i32))
    }

    /// Set when to trigger this interrupt.
    fn setup_source_edge(&mut self, rising: bool, falling: bool) -> HalResult<()> {
        hal_call!(HAL_SetInterruptUpSourceEdge(self.interrupt, rising as i32, falling as i32))
    }

    /// Register a synchronous interrupt. This allows the user to wait on this interrupt.
    pub fn register_sync_interrupt(&mut self) -> HalResult<()> {
        hal_call!(HAL_RequestInterrupts(self.interrupt,
                                        self.input.get_handle(),
                                        mem::transmute(0)))?;
        self.setup_source_edge(true, false)
    }

    /// Register an asynchronous callback to be called when this interrupt is triggered.
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

    /// Wait until this interrupt is triggered, blocking the current thread. Calling this while
    /// trying to read the input from another thread will break.
    pub unsafe fn wait(&mut self, timeout: f64, ignore_previous: bool) -> HalResult<WaitResult> {
        #[allow(unused_unsafe)]
        match hal_call!(HAL_WaitForInterrupt(self.interrupt, timeout, ignore_previous as i32))? {
            0x0 => Ok(WaitResult::Timeout),
            0x1 => Ok(WaitResult::RisingEdge),
            0x100 => Ok(WaitResult::FallingEdge),
            0x101 => Ok(WaitResult::Both),
            _ => Err(HalError(0)),
        }
    }
}

impl Drop for Interrupt {
    fn drop(&mut self) {
        let _ = hal_call!(HAL_CleanInterrupts(self.interrupt));
    }
}
