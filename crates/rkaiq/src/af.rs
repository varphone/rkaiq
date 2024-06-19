//! 自动聚焦
//!
//! AF 模块的功能是指调整相机镜头，使被拍物成像清晰的过程。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{OpMode, XCamResult};

pub trait AutoFocus {
    fn get_focus_mode(&self) -> XCamResult<OpMode>;
    fn set_focus_mode(&self, mode: OpMode) -> XCamResult<()>;
}

impl AutoFocus for Context {
    fn get_focus_mode(&self) -> XCamResult<OpMode> {
        let mut mode: ffi::opMode_t = ffi::opMode_t::OP_AUTO;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_getFocusMode(
                self.internal.as_ptr(),
                &mut mode,
            ))
            .ok()
            .map(|_| mode.into())
        }
    }

    fn set_focus_mode(&self, mode: OpMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_setFocusMode(
                self.internal.as_ptr(),
                mode.into(),
            ))
            .ok()
        }
    }
}
