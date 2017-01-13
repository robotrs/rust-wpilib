use wpilib::wpilib_hal::*;
use wpilib::hal_call::HalResult;
use wpilib::sensor::WaitResult;
use std::{mem, os};

// TODO(Kyle) This whole module is not particularly user-friendly. Abstract it out into nicer code.

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
/// The trigger condition for an analog trigger
pub enum AnalogTriggerType {
    /// Trigger when in a window
    InWindow = 0,
    /// ???
    State = 1,
    /// Trigger on a rising pulse
    RisingPulse = 2,
    /// Trigger on a falling pulse
    FallingPulse = 3,
}

/// Set when to trigger an interrupt.
pub fn setup_source_edge(interrupt: HAL_InterruptHandle,
                         rising: bool,
                         falling: bool)
                         -> HalResult<()> {
    if interrupt == 0 {
        Err(0)
    } else {
        hal_call!(HAL_SetInterruptUpSourceEdge(interrupt, rising as i32, falling as i32))
    }
}

/// Create a new interrupt.
pub fn allocate_interrupts(watcher: bool) -> HalResult<HAL_InterruptHandle> {
    hal_call!(HAL_InitializeInterrupts(watcher as i32))
}

/// Create an interrupt, and trigger `handler` whenever the interrupt is fired.
pub fn request_interrupts_async(handler: HAL_InterruptHandlerFunction,
                                param: *mut os::raw::c_void,
                                port_handle: HAL_Handle,
                                analog_trigger_type: AnalogTriggerType)
                                -> HalResult<HAL_InterruptHandle> {
    let handle = allocate_interrupts(false)?;
    hal_call!(HAL_RequestInterrupts(handle, port_handle, mem::transmute(analog_trigger_type)))?;
    setup_source_edge(port_handle, true, false)?;
    hal_call!(HAL_AttachInterruptHandler(handle, handler, param))?;
    Ok(handle)
}

/// Request an interrupt in synchronous mode. You will then have to explicitly wait on the
/// interrupt when you want to use it.
pub fn request_interrupts_sync(port_handle: HAL_Handle,
                               analog_trigger_type: AnalogTriggerType)
                               -> HalResult<HAL_InterruptHandle> {
    let handle = allocate_interrupts(true)?;
    hal_call!(HAL_RequestInterrupts(handle, port_handle, mem::transmute(analog_trigger_type)))?;
    setup_source_edge(handle, true, false)?;
    Ok(handle)
}

/// Finish using an interrupt.
pub fn cancel_interrupts(interrupt: HAL_InterruptHandle) -> HalResult<()> {
    if interrupt == 0 {
        Err(0)
    } else {
        hal_call!(HAL_CleanInterrupts(interrupt))
    }
}

/// Wait on an interrupt. Calling this while trying to read a sensor from another thread will break
/// your code.
pub unsafe fn wait_for_interrupt(interrupt: HAL_InterruptHandle,
                                 timeout: f64,
                                 ignore_previous: bool)
                                 -> HalResult<WaitResult> {
    #[allow(unused_unsafe)]
    match hal_call!(HAL_WaitForInterrupt(interrupt, timeout, ignore_previous as i32))? {
        0x0 => Ok(WaitResult::Timeout),
        0x1 => Ok(WaitResult::RisingEdge),
        0x100 => Ok(WaitResult::FallingEdge),
        0x101 => Ok(WaitResult::Both),
        _ => Err(0),
    }
}
