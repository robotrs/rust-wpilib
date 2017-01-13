use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::sensor;
use wpilib::usage::*;
use std::{thread, time};

/// An analog input on the RoboRIO
pub struct AnalogInput {
    channel: i32,
    handle: HAL_AnalogInputHandle,
    accumulator_offset: i64,
}

impl AnalogInput {
    /// Create a new analog input on the specified channel, returning an error if initialization
    /// fails.
    pub fn new(channel: i32) -> HalResult<AnalogInput> {
        if !sensor::check_analog_input_channel(channel) {
            return Err(0);
        }

        let port = hal_call!(HAL_InitializeAnalogInputPort(HAL_GetPort(channel)))?;

        report_usage(ResourceType::AnalogChannel, channel);

        Ok(AnalogInput {
            channel: channel,
            handle: port,
            accumulator_offset: 0,
        })
    }

    /// Read a value from the analog input.
    pub fn get_value(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogValue(self.handle))
    }

    /// Read the average value of the analog input over some defined time period.
    pub fn get_average_value(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogAverageValue(self.handle))
    }

    /// Read the raw value of the analog input in volts.
    pub fn get_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogVoltage(self.handle))
    }

    /// Read the average raw value of the analog input in volts over some defined time period.
    pub fn get_average_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogAverageVoltage(self.handle))
    }

    /// Get the factory scaling LSB weight constant:
    /// voltage = ((lsb_weight * 1e-9) * raw) - (offset * 1e-9)
    pub fn get_lsb_weight(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogLSBWeight(self.handle))
    }

    /// Get the factory scaling offset constant:
    /// voltage = ((lsb_weight * 1e-9) * raw) - (offset * 1e-9)
    pub fn get_offset(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogOffset(self.handle))
    }

    /// Get the channel number for this analog input.
    pub fn get_channel(&self) -> i32 {
        self.channel
    }

    /// Set the number of bits to use in averaging. Averaging will sample 2^bits actual reads.
    pub fn set_average_bits(&mut self, bits: i32) -> HalResult<()> {
        hal_call!(HAL_SetAnalogAverageBits(self.handle, bits))
    }

    /// Get the previously-set number of average bits.
    pub fn get_average_bits(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogAverageBits(self.handle))
    }

    /// Set the number of bits to use in oversampling to improve resolution with a slower rate.
    /// Oversampling will use 2^bits actual reads.
    pub fn set_oversample_bits(&mut self, bits: i32) -> HalResult<()> {
        hal_call!(HAL_SetAnalogOversampleBits(self.handle, bits))
    }

    /// Get the previously-set number of oversample bits.
    pub fn get_oversample_bits(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogOversampleBits(self.handle))
    }

    /// Is this analog input attached to an accumulator?
    pub fn is_accumulator_channel(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_IsAccumulatorChannel(self.handle))? != 0)
    }

    /// Initialize an accumulator on this channel.
    pub fn init_accumulator(&mut self) -> HalResult<()> {
        hal_call!(HAL_InitAccumulator(self.handle))
    }

    /// Set the offset for the accumulator.
    pub fn set_accumulator_value(&mut self, value: i64) {
        self.accumulator_offset = value;
    }

    /// Reset the accumulator and wait for the next sample.
    pub fn reset_accumulator(&mut self) -> HalResult<()> {
        hal_call!(HAL_ResetAccumulator(self.handle))?;

        let sample_time = 1f64 / AnalogInput::get_sample_rate()? as f64;
        let over_samples = 1 << self.get_oversample_bits()?;
        let average_samples = 1 << self.get_average_bits()?;
        thread::sleep(time::Duration::from_millis(
            ((1000 * over_samples * average_samples) as f64 *
             sample_time) as u64));
        Ok(())
    }

    /// Set the center of the accumulator. This value will be subtracted from all accumulated
    /// reads.
    pub fn set_accumulator_center(&mut self, center: i32) -> HalResult<()> {
        hal_call!(HAL_SetAccumulatorCenter(self.handle, center))
    }

    /// Set the deadband for the accumulator. Anything within `deadband` of the accumulator center
    /// will be ignored in the accumulator.
    pub fn set_accumulator_deadband(&mut self, deadband: i32) -> HalResult<()> {
        hal_call!(HAL_SetAccumulatorDeadband(self.handle, deadband))
    }

    /// Get a value from the accumulator.
    pub fn get_accumulator_value(&self) -> HalResult<i64> {
        hal_call!(HAL_GetAccumulatorValue(self.handle))
    }

    /// Get the number of accumulated values.
    pub fn get_accumulator_count(&self) -> HalResult<i64> {
        hal_call!(HAL_GetAccumulatorCount(self.handle))
    }

    /// Read the accumulator's value and the count of samples at the same time.
    pub fn get_accumulator_output(&self, value: &mut i64, count: &mut i64) -> HalResult<()> {
        hal_call!(HAL_GetAccumulatorOutput(self.handle, value as *mut i64, count as *mut i64))
    }

    /// Set the sample rate for analog inputs.
    pub fn set_sample_rate(samples_per_second: f64) -> HalResult<()> {
        hal_call!(HAL_SetAnalogSampleRate(samples_per_second))
    }

    /// Get the sample rate for analog inputs.
    pub fn get_sample_rate() -> HalResult<f64> {
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
