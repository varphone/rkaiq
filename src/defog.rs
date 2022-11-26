//! 图像去雾
//!
//! Defog 是通过动态的改变图象的对比度和亮度来实现的去雾增强。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{OpMode, XCamResult};

pub trait Defog {
    fn enable_dhz(&self) -> XCamResult<()>;
    fn disable_dhz(&self) -> XCamResult<()>;

    fn get_dhz_mode(&self) -> XCamResult<OpMode>;
    fn set_dhz_mode(&self, mode: OpMode) -> XCamResult<()>;
}

impl Defog for Context {
    fn enable_dhz(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi2_enableDhz(self.internal.as_ptr())).ok() }
    }

    fn disable_dhz(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi2_disableDhz(self.internal.as_ptr())).ok() }
    }

    fn get_dhz_mode(&self) -> XCamResult<OpMode> {
        let mut mode: ffi::opMode_t = ffi::opMode_t::OP_AUTO;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_getDhzMode(
                self.internal.as_ptr(),
                &mut mode,
            ))
            .ok()
            .map(|_| mode.into())
        }
    }

    fn set_dhz_mode(&self, mode: OpMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_setDhzMode(
                self.internal.as_ptr(),
                mode.into(),
            ))
            .ok()
        }
    }
}
