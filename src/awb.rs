//! 自动白平衡
//!
//! AWB 模块的功能是通过改变拍摄设备的色彩通道的增益，
//！对色温环境所造成的颜色偏差和拍摄设备本身所固有的色彩通道增益的偏差进行统一补偿，
//！从而让获得的图像能正确反映物体的真实色彩。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{OpMode, XCamResult};

pub trait AutoWhiteBalance {
    fn get_wb_mode(&self) -> XCamResult<OpMode>;
    fn set_wb_mode(&self, mode: OpMode) -> XCamResult<()>;
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
