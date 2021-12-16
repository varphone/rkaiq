//! 自动白平衡
//!
//! AWB 模块的功能是通过改变拍摄设备的色彩通道的增益，
//！对色温环境所造成的颜色偏差和拍摄设备本身所固有的色彩通道增益的偏差进行统一补偿，
//！从而让获得的图像能正确反映物体的真实色彩。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{OpMode, WbGain, WbScene, XCamResult};

/// 一个描述自动白平衡的契定。
pub trait AutoWhiteBalance {
    /// 获取白平衡工作模式。
    fn get_wb_mode(&self) -> XCamResult<OpMode>;

    /// 设置白平衡工作模式。
    fn set_wb_mode(&self, mode: OpMode) -> XCamResult<()>;

    /// 锁定当前白平衡参数。
    fn lock_awb(&self) -> XCamResult<()>;

    /// 解锁已被锁定的白平衡参数。
    fn unlock_awb(&self) -> XCamResult<()>;

    /// 获取白平衡场景。
    fn get_mwb_scene(&self) -> XCamResult<WbScene>;

    /// 设置白平衡场景。
    fn set_mwb_scene<T: Into<WbScene>>(&self, scene: T) -> XCamResult<()>;

    /// 获取白平衡增益系数。
    fn get_mwb_gain(&self) -> XCamResult<WbGain>;

    /// 设置白平衡增益系数。
    fn set_mwb_gain<T: Into<WbGain>>(&self, gain: T) -> XCamResult<()>;

    /// 获取白平衡色温参数。
    fn get_mwb_ct(&self) -> XCamResult<u32>;

    /// 设置白平衡色温参数。
    fn set_mwb_ct(&self, ct: u32) -> XCamResult<()>;
}

impl AutoWhiteBalance for Context {
    fn get_wb_mode(&self) -> XCamResult<OpMode> {
        let mut mode = ffi::opMode_t::OP_INVAL;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getWBMode(
                self.internal.as_ptr(),
                &mut mode,
            ))
            .ok()
            .map(|_| mode.into())
        }
    }

    fn set_wb_mode(&self, mode: OpMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setWBMode(
                self.internal.as_ptr(),
                mode.into(),
            ))
            .ok()
        }
    }

    fn lock_awb(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi_lockAWB(self.internal.as_ptr())).ok() }
    }

    fn unlock_awb(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi_unlockAWB(self.internal.as_ptr())).ok() }
    }

    fn get_mwb_scene(&self) -> XCamResult<WbScene> {
        let mut scene: WbScene = Default::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getMWBScene(
                self.internal.as_ptr(),
                &mut scene,
            ))
            .ok()
            .map(|_| scene)
        }
    }

    fn set_mwb_scene<T: Into<WbScene>>(&self, scene: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setMWBScene(
                self.internal.as_ptr(),
                scene.into(),
            ))
            .ok()
        }
    }

    fn get_mwb_gain(&self) -> XCamResult<WbGain> {
        let mut gain: WbGain = Default::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getMWBGain(
                self.internal.as_ptr(),
                &mut gain,
            ))
            .ok()
            .map(|_| gain)
        }
    }

    fn set_mwb_gain<T: Into<WbGain>>(&self, gain: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setMWBGain(
                self.internal.as_ptr(),
                &mut gain.into(),
            ))
            .ok()
        }
    }

    fn get_mwb_ct(&self) -> XCamResult<u32> {
        let mut ct: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getMWBCT(self.internal.as_ptr(), &mut ct))
                .ok()
                .map(|_| ct)
        }
    }

    fn set_mwb_ct(&self, ct: u32) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi_setMWBCT(self.internal.as_ptr(), ct)).ok() }
    }
}

pub enum WbOpMode {
    #[cfg(feature = "rel_1_0")]
    Invalid,
    Manual,
    Auto,
    Max,
}

impl From<ffi::opMode_t> for WbOpMode {
    fn from(val: ffi::opMode_t) -> Self {
        use ffi::opMode_t::*;
        match val {
            OP_AUTO => WbOpMode::Auto,
            #[cfg(feature = "rel_1_0")]
            OP_SEMI_AUTO => WbOpMode::Invalid,
            #[cfg(not(feature = "rel_1_0"))]
            OP_SEMI_AUTO => WbOpMode::Max,
            OP_MANUAL => WbOpMode::Manual,
            #[cfg(feature = "rel_1_0")]
            _ => WbOpMode::Invalid,
            #[cfg(not(feature = "rel_1_0"))]
            _ => WbOpMode::Max,
        }
    }
}

impl From<ffi::rk_aiq_wb_op_mode_t> for WbOpMode {
    fn from(val: ffi::rk_aiq_wb_op_mode_t) -> Self {
        use ffi::rk_aiq_wb_op_mode_t::*;
        match val {
            #[cfg(feature = "rel_1_0")]
            RK_AIQ_WB_MODE_INVALID => WbOpMode::Invalid,
            RK_AIQ_WB_MODE_MANUAL => WbOpMode::Manual,
            RK_AIQ_WB_MODE_AUTO => WbOpMode::Auto,
            RK_AIQ_WB_MODE_MAX => WbOpMode::Max,
        }
    }
}

impl From<WbOpMode> for ffi::opMode_t {
    fn from(val: WbOpMode) -> Self {
        use ffi::opMode_t::*;
        match val {
            #[cfg(feature = "rel_1_0")]
            WbOpMode::Invalid => OP_INVAL,
            WbOpMode::Manual => OP_MANUAL,
            WbOpMode::Auto => OP_AUTO,
            WbOpMode::Max => OP_INVAL,
        }
    }
}

impl From<WbOpMode> for ffi::rk_aiq_wb_op_mode_t {
    fn from(val: WbOpMode) -> Self {
        use ffi::rk_aiq_wb_op_mode_t::*;
        match val {
            #[cfg(feature = "rel_1_0")]
            WbOpMode::Invalid => RK_AIQ_WB_MODE_INVALID,
            WbOpMode::Manual => RK_AIQ_WB_MODE_MANUAL,
            WbOpMode::Auto => RK_AIQ_WB_MODE_AUTO,
            WbOpMode::Max => RK_AIQ_WB_MODE_MAX,
        }
    }
}
