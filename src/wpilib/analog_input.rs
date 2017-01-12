use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::sensor;
use std::{ptr, thread, time};

pub struct AnalogInput {
    channel: i32,
    handle: HAL_AnalogInputHandle,
    accumulator_offset: i64,
}

impl AnalogInput {
    pub fn new(channel: i32) -> HalResult<AnalogInput> {
        if !sensor::check_analog_input_channel(channel) {
            return Err(0);
        }

        let port = hal_call!(HAL_InitializeAnalogInputPort(HAL_GetPort(channel)))?;

        unsafe {
            HAL_Report(6, channel, 0, ptr::null());
        }

        Ok(AnalogInput {
            channel: channel,
            handle: port,
            accumulator_offset: 0,
        })
    }

    pub fn get_value(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogValue(self.handle))
    }

    pub fn get_average_value(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogAverageValue(self.handle))
    }

    pub fn get_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogVoltage(self.handle))
    }

    pub fn get_average_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogAverageVoltage(self.handle))
    }

    pub fn get_lsb_weight(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogLSBWeight(self.handle))
    }

    pub fn get_offset(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogOffset(self.handle))
    }

    pub fn get_channel(&self) -> i32 {
        self.channel
    }

    pub fn set_average_bits(&mut self, bits: i32) -> HalResult<()> {
        hal_call!(HAL_SetAnalogAverageBits(self.handle, bits))
    }

    pub fn get_average_bits(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogAverageBits(self.handle))
    }

    pub fn set_oversample_bits(&mut self, bits: i32) -> HalResult<()> {
        hal_call!(HAL_SetAnalogOversampleBits(self.handle, bits))
    }

    pub fn get_oversample_bits(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogOversampleBits(self.handle))
    }

    pub fn is_accumulator_channel(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_IsAccumulatorChannel(self.handle))? != 0)
    }

    pub fn init_accumulator(&mut self) -> HalResult<()> {
        hal_call!(HAL_InitAccumulator(self.handle))
    }

    pub fn set_accumulator_value(&mut self, value: i64) {
        self.accumulator_offset = value;
    }

    pub fn reset_accumulator(&mut self) -> HalResult<()> {
        hal_call!(HAL_ResetAccumulator(self.handle))?;

        let sample_time = 1f64 / self.get_sample_rate()? as f64;
        let over_samples = 1 << self.get_oversample_bits()?;
        let average_samples = 1 << self.get_average_bits()?;
        thread::sleep(time::Duration::from_millis(
            ((1000 * over_samples * average_samples) as f64 *
             sample_time) as u64));
        Ok(())
    }

    pub fn set_accumulator_center(&mut self, center: i32) -> HalResult<()> {
        hal_call!(HAL_SetAccumulatorCenter(self.handle, center))
    }

    pub fn set_accumulator_deadband(&mut self, deadband: i32) -> HalResult<()> {
        hal_call!(HAL_SetAccumulatorDeadband(self.handle, deadband))
    }

    pub fn get_accumulator_value(&self) -> HalResult<i64> {
        hal_call!(HAL_GetAccumulatorValue(self.handle))
    }

    pub fn get_accumulator_count(&self) -> HalResult<i64> {
        hal_call!(HAL_GetAccumulatorCount(self.handle))
    }

    pub fn get_accumulator_output(&self, value: &mut i64, count: &mut i64) -> HalResult<()> {
        hal_call!(HAL_GetAccumulatorOutput(self.handle, value as *mut i64, count as *mut i64))
    }

    pub fn set_sample_rate(&mut self, samples_per_second: f64) -> HalResult<()> {
        hal_call!(HAL_SetAnalogSampleRate(samples_per_second))
    }

    pub fn get_sample_rate(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogSampleRate())
    }
}

impl Drop for AnalogInput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeAnalogInputPort(self.handle);
        }
    }
}
