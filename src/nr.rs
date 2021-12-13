//! 自动白平衡
//!
//! AWB 模块的功能是通过改变拍摄设备的色彩通道的增益，
//！对色温环境所造成的颜色偏差和拍摄设备本身所固有的色彩通道增益的偏差进行统一补偿，
//！从而让获得的图像能正确反映物体的真实色彩。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{OpMode, XCamResult};

pub trait NoiseRemoval {
    fn get_nr_mode(&self) -> XCamResult<OpMode>;
    fn set_nr_mode(&self, mode: OpMode) -> XCamResult<()>;

    fn get_anr_strength(&self) -> XCamResult<u32>;
    fn set_anr_strength(&self, strength: u32) -> XCamResult<()>;

    fn get_ms_nr_strength(&self) -> XCamResult<(bool, u32)>;
    fn set_ms_nr_strength(&self, on: bool, strength: u32) -> XCamResult<()>;

    fn get_mt_nr_strength(&self) -> XCamResult<(bool, u32)>;
    fn set_mt_nr_strength(&self, on: bool, strength: u32) -> XCamResult<()>;
}

impl NoiseRemoval for Context {
    fn get_nr_mode(&self) -> XCamResult<OpMode> {
        let mut mode = ffi::opMode_t::OP_INVAL;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getNRMode(
                self.internal.as_ptr(),
                &mut mode,
            ))
            .ok()
            .map(|_| mode.into())
        }
    }

    fn set_nr_mode(&self, mode: OpMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setNRMode(
                self.internal.as_ptr(),
                mode.into(),
            ))
            .ok()
        }
    }

    fn get_anr_strength(&self) -> XCamResult<u32> {
        let mut strength: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getANRStrth(
                self.internal.as_ptr(),
                &mut strength,
            ))
            .ok()
            .map(|_| strength)
        }
    }

    fn set_anr_strength(&self, strength: u32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setANRStrth(
                self.internal.as_ptr(),
                strength,
            ))
            .ok()
        }
    }

    fn get_ms_nr_strength(&self) -> XCamResult<(bool, u32)> {
        let mut on: bool = false;
        let mut strength: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getMSpaNRStrth(
                self.internal.as_ptr(),
                &mut on,
                &mut strength,
            ))
            .ok()
            .map(|_| (on, strength))
        }
    }

    fn set_ms_nr_strength(&self, on: bool, strength: u32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setMSpaNRStrth(
                self.internal.as_ptr(),
                on,
                strength,
            ))
            .ok()
        }
    }

    fn get_mt_nr_strength(&self) -> XCamResult<(bool, u32)> {
        let mut on: bool = false;
        let mut strength: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getMTNRStrth(
                self.internal.as_ptr(),
                &mut on,
                &mut strength,
            ))
            .ok()
            .map(|_| (on, strength))
        }
    }

    fn set_mt_nr_strength(&self, on: bool, strength: u32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setMTNRStrth(
                self.internal.as_ptr(),
                on,
                strength,
            ))
            .ok()
        }
    }
}
