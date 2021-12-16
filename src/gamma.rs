//! Gamma
//!
//! Gamma 模块对图像进行亮度空间非线性转换以适配输出设备。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{GammaAttr, XCamResult};

/// 一个描述 Gamma 控制的契定。
pub trait Gamma {
    /// 设置伽玛。
    fn set_gamma_coef<T: Into<GammaAttr>>(&self, gamma_attr: T) -> XCamResult<()>;
}

impl Gamma for Context {
    fn set_gamma_coef<T: Into<GammaAttr>>(&self, gamma_attr: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setGammaCoef(
                self.internal.as_ptr(),
                gamma_attr.into(),
            ))
            .ok()
        }
    }
}
