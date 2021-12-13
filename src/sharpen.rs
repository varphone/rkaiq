//! 图像锐化
//!
//! Sharpen 模块用于增强图像的清晰度，包括调节图像边缘的锐化属性和增强图像的细节和纹理。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::XCamResult;

pub trait Sharpen {
    fn get_sharpness(&self) -> XCamResult<u32>;
    fn set_sharpness(&self, mode: u32) -> XCamResult<()>;
}

impl Sharpen for Context {
    fn get_sharpness(&self) -> XCamResult<u32> {
        let mut level: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getSharpness(
                self.internal.as_ptr(),
                &mut level,
            ))
            .ok()
            .map(|_| level)
        }
    }

    fn set_sharpness(&self, level: u32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setSharpness(self.internal.as_ptr(), level)).ok()
        }
    }
}
