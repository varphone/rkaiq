//! 杂项
//!
//! 未分类的功能、接口等。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{FrameRateInfo, GrayMode, XCamResult};

/// 一个描述杂项控制的契定。
pub trait Miscellaneous {
    /// 获取黑白图像模式的工作方式。
    fn get_gray_mode(&self) -> GrayMode;

    /// 设置黑白图像模式的工作方式。
    fn set_gray_mode<T: Into<GrayMode>>(&self, mode: T) -> XCamResult<()>;

    /// 获取图像输出帧率信息。
    fn get_frame_rate(&self) -> XCamResult<FrameRateInfo>;

    /// 设置图像输出帧率。
    fn set_frame_rate<T: Into<FrameRateInfo>>(&self, mode: T) -> XCamResult<()>;

    /// 获取图像镜像、翻转信息。
    fn get_mirror_flip(&self) -> XCamResult<(bool, bool)>;

    /// 设置图像镜像、翻转。
    fn set_mirror_flip(&self, mirror: bool, flip: bool, skip_frm_cnt: i32) -> XCamResult<()>;
}

impl Miscellaneous for Context {
    fn get_gray_mode(&self) -> GrayMode {
        unsafe { ffi::rk_aiq_uapi_getGrayMode(self.internal.as_ptr()) }
    }

    fn set_gray_mode<T: Into<GrayMode>>(&self, mode: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setGrayMode(
                self.internal.as_ptr(),
                mode.into(),
            ))
            .ok()
        }
    }

    fn get_frame_rate(&self) -> XCamResult<FrameRateInfo> {
        let mut info: FrameRateInfo = Default::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getFrameRate(
                self.internal.as_ptr(),
                &mut info,
            ))
            .ok()
            .map(|_| info)
        }
    }

    fn set_frame_rate<T: Into<FrameRateInfo>>(&self, info: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setFrameRate(
                self.internal.as_ptr(),
                info.into(),
            ))
            .ok()
        }
    }

    fn get_mirror_flip(&self) -> XCamResult<(bool, bool)> {
        let mut mirror: bool = false;
        let mut flip: bool = false;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getMirrorFlip(
                self.internal.as_ptr(),
                &mut mirror,
                &mut flip,
            ))
            .ok()
            .map(|_| (mirror, flip))
        }
    }

    fn set_mirror_flip(&self, mirror: bool, flip: bool, skip_frm_cnt: i32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setMirroFlip(
                self.internal.as_ptr(),
                mirror,
                flip,
                skip_frm_cnt,
            ))
            .ok()
        }
    }
}
