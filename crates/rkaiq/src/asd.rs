//! 环境光强检测
//!
//! 通过图像信息计算当前环境亮度。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{AsdAttrib, XCamResult};

/// 一个描述环境光强检测的契定。
pub trait AmbientDetection {
    /// 获取当前环境亮度的计算结果。
    fn get_asd_attrib(&self) -> XCamResult<AsdAttrib>;
}

impl AmbientDetection for Context {
    fn get_asd_attrib(&self) -> XCamResult<AsdAttrib> {
        let mut attr: AsdAttrib = Default::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_user_api_asd_GetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()
            .map(|_| attr)
        }
    }
}
