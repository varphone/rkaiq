//! 高动态范围成像
//!
//! HDR(高动态范围成像, High Dynamic Range Imaging，HDRI 或 HDR)，
/// 在计算机图形学与摄影中，是通过计算实现使用现有设备获得比普通数字图像
/// 技术更大曝光动态范围（即更大的明暗差别）图像的一种技术。
/// HDR 的目的就是要正确地还原出超出现有设备动态范围的现实场景光亮比例。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{OpMode, XCamResult};

pub trait HighDynamicRange {
    /// 获取 HDR 工作模式。
    fn get_hdr_mode(&self) -> XCamResult<OpMode>;

    /// 设置 HDR 工作模式。
    fn set_hdr_mode<T: Into<OpMode>>(&self, mode: T) -> XCamResult<()>;

    /// 设置手动模式下的 HDR 强度。
    fn get_hdr_strth(&self) -> XCamResult<(bool, u32)>;

    /// 设置手动模式下的 HDR 强度。
    fn set_hdr_strth(&self, enabled: bool, level: u32) -> XCamResult<()>;
}

impl HighDynamicRange for Context {
    fn get_hdr_mode(&self) -> XCamResult<OpMode> {
        #[cfg(feature = "v2_0")]
        unsafe {
            let mut mode: ffi::opMode_t = Default::default();
            XCamError::from(ffi::rk_aiq_uapi_getHDRMode(
                self.internal.as_ptr(),
                &mut mode,
            ))
            .ok()
            .map(|_| mode.into())
        }
        #[cfg(feature = "v3_0")]
        Ok(OpMode::Auto)
    }

    #[cfg(feature = "v2_0")]
    fn set_hdr_mode<T: Into<OpMode>>(&self, mode: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setHDRMode(
                self.internal.as_ptr(),
                mode.into().into(),
            ))
            .ok()
        }
    }

    #[cfg(feature = "v3_0")]
    fn set_hdr_mode<T: Into<OpMode>>(&self, _mode: T) -> XCamResult<()> {
        Ok(())
    }

    fn get_hdr_strth(&self) -> XCamResult<(bool, u32)> {
        let mut enabled: bool = false;
        let mut level: u32 = 0;
        #[cfg(feature = "v2_0")]
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getMHDRStrth(
                self.internal.as_ptr(),
                &mut enabled,
                &mut level,
            ))
            .ok()
            .map(|_| (enabled, level))
        }
        #[cfg(feature = "v3_0")]
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_getMHDRStrth(
                self.internal.as_ptr(),
                &mut enabled,
                &mut level,
            ))
            .ok()
            .map(|_| (enabled, level))
        }
    }

    fn set_hdr_strth(&self, enabled: bool, level: u32) -> XCamResult<()> {
        #[cfg(feature = "v2_0")]
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setMHDRStrth(
                self.internal.as_ptr(),
                enabled,
                level,
            ))
            .ok()
        }
        #[cfg(feature = "v3_0")]
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_setMHDRStrth(
                self.internal.as_ptr(),
                enabled,
                level,
            ))
            .ok()
        }
    }
}
