//! 鱼眼畸变校正
//!
//! 光学系统、电子扫描系统失真而引起的斜视畸变、枕形、桶形畸变等，都可能使图像产生几何特性失真。
//! 图像的畸变矫正是以某种变换方式将畸变图像转换为理想图像的过程。
//! 该模块对x和y方向的图像畸变进行校正。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::XCamResult;

pub trait FEC {
    fn enable_fec(&self) -> XCamResult<()>;
    fn disable_fec(&self) -> XCamResult<()>;
}

impl FEC for Context {
    fn enable_fec(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi2_setFecEn(self.internal.as_ptr(), true)).ok() }
    }

    fn disable_fec(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi2_setFecEn(self.internal.as_ptr(), false)).ok() }
    }
}
