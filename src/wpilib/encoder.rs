use wpilib::wpilib_hal::*;
use wpilib::hal_call::*;
use wpilib::digital_input::DigitalInput;
use std::ptr;
use std::mem;

pub enum IndexingType {
    ResetWhileHigh,
    ResetWhileLow,
    ResetOnFallingEdge,
    ResetOnRisingEdge,
}

pub struct Encoder {
    source_a: DigitalInput,
    source_b: DigitalInput,
    source_index: Option<DigitalInput>,
    encoder: HAL_EncoderHandle,
}

impl Encoder {
    pub fn new(channel_a: i32,
               channel_b: i32,
               encoding: HAL_EncoderEncodingType)
               -> HalResult<Encoder> {
        let source_a = DigitalInput::new(channel_a)?;
        let source_b = DigitalInput::new(channel_b)?;

        let handle = hal_call!(HAL_InitializeEncoder(source_a.get_handle(),
                                                     mem::transmute(0),
                                                     source_b.get_handle(),
                                                     mem::transmute(0),
                                                     false as i32,
                                                     encoding))?;
        let encoder = Encoder {
            source_a: source_a,
            source_b: source_b,
            source_index: None,
            encoder: handle,
        };
        unsafe {
            HAL_Report(18, encoder.get_fpga_index()?, encoding as i32, ptr::null());
        }
        Ok(encoder)
    }

    pub fn get_fpga_index(&self) -> HalResult<i32> {
        hal_call!(HAL_GetEncoderFPGAIndex(self.encoder))
    }

    pub fn get(&self) -> HalResult<i32> {
        hal_call!(HAL_GetEncoder(self.encoder))
    }

    pub fn get_raw(&self) -> HalResult<i32> {
        hal_call!(HAL_GetEncoderRaw(self.encoder))
    }

    pub fn get_encoding_scale(&self) -> HalResult<i32> {
        hal_call!(HAL_GetEncoderEncodingScale(self.encoder))
    }

    pub fn get_rate(&self) -> HalResult<f64> {
        hal_call!(HAL_GetEncoderRate(self.encoder))
    }

    pub fn set_min_rate(&mut self, min_rate: f64) -> HalResult<()> {
        hal_call!(HAL_SetEncoderMinRate(self.encoder, min_rate))
    }

    pub fn reset(&mut self) -> HalResult<()> {
        hal_call!(HAL_ResetEncoder(self.encoder))
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        hal_call!(HAL_FreeEncoder(self.encoder)).unwrap();
    }
}
