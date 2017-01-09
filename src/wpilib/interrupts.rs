use wpilib::wpilib_hal::*;
use std::mem;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum AnalogTriggerType {
    InWindow = 0,
    State = 1,
    RisingPulse = 2,
    FallingPulse = 3,
}

pub fn setup_source_edge(interrupt: HAL_InterruptHandle,
                         rising: bool,
                         falling: bool)
                         -> Result<(), i32> {
    if interrupt == 0 {
        Err(0)
    } else {
        hal_call!(HAL_SetInterruptUpSourceEdge(interrupt, rising as i32, falling as i32))
    }
}

pub fn allocate_interrupts(watcher: bool) -> Result<HAL_InterruptHandle, i32> {
    hal_call!(HAL_InitializeInterrupts(watcher as i32))
}

pub fn request_interrupts_async(handler: HAL_InterruptHandlerFunction,
                                param: *mut (),
                                port_handle: HAL_Handle,
                                analog_trigger_type: AnalogTriggerType)
                                -> Result<HAL_InterruptHandle, i32> {
    let handle = allocate_interrupts(false)?;
    hal_call!(HAL_RequestInterrupts(handle, port_handle, mem::transmute(analog_trigger_type)))?;
    Ok(handle)
}

pub fn request_interrupts_sync(port_handle: HAL_Handle,
                               analog_trigger_type: AnalogTriggerType)
                               -> Result<HAL_InterruptHandle, i32> {
    let handle = allocate_interrupts(true)?;
    hal_call!(HAL_RequestInterrupts(handle, port_handle, mem::transmute(analog_trigger_type)))?;
    setup_source_edge(handle, true, false);
    Ok(handle)
}

pub fn cancel_interrupts(interrupt: HAL_InterruptHandle) -> Result<(), i32> {
    if interrupt == 0 {
        Err(0)
    } else {
        hal_call!(HAL_CleanInterrupts(interrupt))
    }
}
