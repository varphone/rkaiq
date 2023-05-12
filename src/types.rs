use super::error::XCamError;
use super::ffi;

pub type AlgoContext = ffi::RkAiqAlgoContext;
pub type AlgoDescComm = ffi::RkAiqAlgoDesComm;
pub type AntiFlickerMode = ffi::antiFlickerMode_t;
pub type AsdAttrib = ffi::asd_attrib_t;
pub type CpslCfg = ffi::rk_aiq_cpsl_cfg_t;
pub type CpslCap = ffi::rk_aiq_cpsl_cap_t;
pub type CpslInfo = ffi::rk_aiq_cpsl_info_t;
pub type ExpPwrLineFreq = ffi::expPwrLineFreq_t;
pub type FrameRateInfo = ffi::frameRateInfo_t;
#[cfg(feature = "v2_0")]
pub type GammaApiManual = ffi::Agamma_api_manual_t;
#[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
pub type GammaApiManualV21 = ffi::Agamma_api_manualV21_t;
#[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
pub type GammaApiManualV30 = ffi::Agamma_api_manualV30_t;
#[cfg(feature = "v2_0")]
pub type GammaAttr = ffi::rk_aiq_gamma_attrib_t;
#[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
pub type GammaAttr = ffi::rk_aiq_gamma_attr_t;
pub type GammaCaliDb = ffi::CalibDb_Gamma_t;
#[cfg(feature = "v2_0")]
pub type GammaCurveType = ffi::rk_gamma_curve_type_t;
#[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
pub type GammaCurveType = ffi::GammaType_t;
#[cfg(feature = "v2_0")]
pub type GammaCurveUsrDefine1Para = ffi::rk_gamma_curve_usr_define1_para_t;
#[cfg(feature = "v2_0")]
pub type GammaCurveUsrDefine2Para = ffi::rk_gamma_curve_usr_define2_para_t;
#[cfg(feature = "v2_0")]
pub type GammaMode = ffi::rk_aiq_gamma_op_mode_t;
#[cfg(not(feature = "v2_0"))]
pub type GammaMode = ffi::gamma_op_mode_t;
pub type GammaOpMode = ffi::rk_aiq_gamma_op_mode_t;
pub type GrayMode = ffi::rk_aiq_gray_mode_t;
pub type ModuleId = ffi::rk_aiq_module_id_t;
pub type PaRange = ffi::paRange_t;
pub type Rect = ffi::rk_aiq_rect_t;
pub type StaticInfo = ffi::rk_aiq_static_info_t;
pub type WbGain = ffi::rk_aiq_wb_gain_t;
pub type WbScene = ffi::rk_aiq_wb_scene_t;
pub type XCamResult<T> = Result<T, XCamError>;

/// 一个描述自动手动模式的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpMode {
    Auto,
    Manual,
    SemiAuto,
    #[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
    RegManual,
    Invalid,
}

impl From<ffi::opMode_t> for OpMode {
    fn from(val: ffi::opMode_t) -> Self {
        use ffi::opMode_t::*;
        match val {
            OP_AUTO => OpMode::Auto,
            OP_MANUAL => OpMode::Manual,
            OP_SEMI_AUTO => OpMode::SemiAuto,
            #[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
            OP_REG_MANUAL => OpMode::RegManual,
            OP_INVAL => OpMode::Invalid,
        }
    }
}

impl From<OpMode> for ffi::opMode_t {
    fn from(val: OpMode) -> ffi::opMode_t {
        use ffi::opMode_t::*;
        match val {
            OpMode::Auto => OP_AUTO,
            OpMode::Manual => OP_MANUAL,
            OpMode::SemiAuto => OP_SEMI_AUTO,
            #[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
            OpMode::RegManual => OP_REG_MANUAL,
            OpMode::Invalid => OP_INVAL,
        }
    }
}

pub enum WorkingMode {
    Normal,
    IspHdr2,
    IspHdr3,
}

impl From<ffi::rk_aiq_working_mode_t> for WorkingMode {
    fn from(val: ffi::rk_aiq_working_mode_t) -> Self {
        use ffi::rk_aiq_working_mode_t::*;
        match val {
            RK_AIQ_WORKING_MODE_NORMAL => WorkingMode::Normal,
            RK_AIQ_WORKING_MODE_ISP_HDR2 => WorkingMode::IspHdr2,
            RK_AIQ_WORKING_MODE_ISP_HDR3 => WorkingMode::IspHdr3,
        }
    }
}

impl From<WorkingMode> for ffi::rk_aiq_working_mode_t {
    fn from(val: WorkingMode) -> Self {
        use ffi::rk_aiq_working_mode_t::*;
        match val {
            WorkingMode::Normal => RK_AIQ_WORKING_MODE_NORMAL,
            WorkingMode::IspHdr2 => RK_AIQ_WORKING_MODE_ISP_HDR2,
            WorkingMode::IspHdr3 => RK_AIQ_WORKING_MODE_ISP_HDR3,
        }
    }
}

/// 一个代表摄像头朝向的枚举。
#[derive(Copy, Clone, Debug, Default)]
pub enum CameraFacing {
    #[default]
    Back,
    Front,
    Left,
    Right,
}

/// 一个代表摄像头朝向解析错误的枚举。
#[derive(Copy, Clone, Debug)]
pub enum CameraFacingParseError {
    NotMatched,
}

impl std::str::FromStr for CameraFacing {
    type Err = CameraFacingParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "back" => Ok(CameraFacing::Back),
            "f" | "front" => Ok(CameraFacing::Front),
            "l" | "left" => Ok(CameraFacing::Left),
            "r" | "right" => Ok(CameraFacing::Right),
            _ => Err(CameraFacingParseError::NotMatched),
        }
    }
}

/// 一个代表摄像头模块信息的类型。
#[derive(Clone, Debug, Default)]
pub struct CameraModuleInfo {
    /// 摄像头模块编号。
    pub index: usize,
    /// 摄像头朝向。
    pub facing: CameraFacing,
    /// 摄像头模块名称。
    pub name: String,
    /// 摄像头模块所在的总线编号。
    pub bus: usize,
    /// 摄像头模块的通信地址。
    pub reg: usize,
}

impl std::str::FromStr for CameraModuleInfo {
    type Err = CameraModuleInfoParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(
            r#"m([[:xdigit:]]{2})_([bflr]{1})_([[:alnum:]]+)\s+([[:digit:]]+)-([[:xdigit:]]{4})"#,
        )
        .map_err(|_| CameraModuleInfoParseError::RegexError)?;

        let caps = re
            .captures(s)
            .ok_or(CameraModuleInfoParseError::NotMatched)?;

        let index = usize::from_str_radix(&caps[1], 16)
            .map_err(|_| CameraModuleInfoParseError::ParseIndexFailed)?;
        let facing = CameraFacing::from_str(&caps[2])
            .map_err(|_| CameraModuleInfoParseError::ParseFacingFailed)?;
        let name = caps[3].to_owned();
        let bus = usize::from_str_radix(&caps[4], 16)
            .map_err(|_| CameraModuleInfoParseError::ParseBusFailed)?;
        let reg = usize::from_str_radix(&caps[5], 16)
            .map_err(|_| CameraModuleInfoParseError::ParseRegFailed)?;
        Ok(CameraModuleInfo {
            index,
            facing,
            name,
            bus,
            reg,
        })
    }
}

/// 一个代表摄像头模块信息解析错误的枚举。
#[derive(Copy, Clone, Debug)]
pub enum CameraModuleInfoParseError {
    ParseBusFailed,
    ParseFacingFailed,
    ParseIndexFailed,
    ParseRegFailed,
    NotMatched,
    RegexError,
}
