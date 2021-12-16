use super::error::XCamError;
use super::ffi;

pub type AlgoContext = ffi::RkAiqAlgoContext;
pub type AlgoDescComm = ffi::RkAiqAlgoDesComm;
pub type AntiFlickerMode = ffi::antiFlickerMode_t;
pub type CpslCfg = ffi::rk_aiq_cpsl_cfg_t;
pub type CpslCap = ffi::rk_aiq_cpsl_cap_t;
pub type CpslInfo = ffi::rk_aiq_cpsl_info_t;
pub type ExpPwrLineFreq = ffi::expPwrLineFreq_t;
pub type ModuleId = ffi::rk_aiq_module_id_t;
pub type Rect = ffi::rk_aiq_rect_t;
pub type StaticInfo = ffi::rk_aiq_static_info_t;
pub type XCamResult<T> = Result<T, XCamError>;

/// 一个描述自动手动模式的枚举。
pub enum OpMode {
    Auto,
    Manual,
    SemiAuto,
    Invalid,
}

impl From<ffi::opMode_t> for OpMode {
    fn from(val: ffi::opMode_t) -> Self {
        use ffi::opMode_t::*;
        match val {
            OP_AUTO => OpMode::Auto,
            OP_MANUAL => OpMode::Manual,
            OP_SEMI_AUTO => OpMode::SemiAuto,
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
