use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{OpMode, XCamResult};

#[cfg(feature = "rel_1_0")]
pub enum AeMode {
    Auto,
    IrisPrior,
    ShutterPrior,
}

#[cfg(feature = "rel_1_0")]
impl From<ffi::aeMode_t> for AeMode {
    fn from(val: ffi::aeMode_t) -> Self {
        use ffi::aeMode_t::*;
        match val {
            AE_AUTO => Self::Auto,
            AE_IRIS_PRIOR => Self::IrisPrior,
            AE_SHUTTER_PRIOR => Self::ShutterPrior,
        }
    }
}

#[cfg(feature = "rel_1_0")]
impl From<AeMode> for ffi::aeMode_t {
    fn from(val: AeMode) -> Self {
        use ffi::aeMode_t::*;
        match val {
            AeMode::Auto => AE_AUTO,
            AeMode::IrisPrior => AE_IRIS_PRIOR,
            AeMode::ShutterPrior => AE_SHUTTER_PRIOR,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AeMeasAreaType {
    Auto,
    Up,
    Bottom,
    Left,
    Right,
    Center,
}

impl From<ffi::aeMeasAreaType_e> for AeMeasAreaType {
    fn from(val: ffi::aeMeasAreaType_e) -> Self {
        use ffi::aeMeasAreaType_e::*;
        match val {
            AE_MEAS_AREA_AUTO => Self::Auto,
            AE_MEAS_AREA_UP => Self::Up,
            AE_MEAS_AREA_BOTTOM => Self::Bottom,
            AE_MEAS_AREA_LEFT => Self::Left,
            AE_MEAS_AREA_RIGHT => Self::Right,
            AE_MEAS_AREA_CENTER => Self::Center,
        }
    }
}

impl From<AeMeasAreaType> for ffi::aeMeasAreaType_e {
    fn from(val: AeMeasAreaType) -> Self {
        use ffi::aeMeasAreaType_e::*;
        match val {
            AeMeasAreaType::Auto => AE_MEAS_AREA_AUTO,
            AeMeasAreaType::Up => AE_MEAS_AREA_UP,
            AeMeasAreaType::Bottom => AE_MEAS_AREA_BOTTOM,
            AeMeasAreaType::Left => AE_MEAS_AREA_LEFT,
            AeMeasAreaType::Right => AE_MEAS_AREA_RIGHT,
            AeMeasAreaType::Center => AE_MEAS_AREA_CENTER,
        }
    }
}

pub trait AutoExposure {
    #[cfg(feature = "rel_1_0")]
    fn get_ae_mode(&self) -> XCamResult<AeMode>;
    #[cfg(feature = "rel_1_0")]
    fn set_ae_mode(&self, mode: AeMode) -> XCamResult<()>;

    fn get_exp_mode(&self) -> XCamResult<OpMode>;
    fn set_exp_mode(&self, mode: OpMode) -> XCamResult<()>;

    fn get_exp_gain_range(&self) -> XCamResult<(f32, f32)>;
    fn set_exp_gain_range(&self, min: f32, max: f32) -> XCamResult<()>;

    fn get_exp_time_range(&self) -> XCamResult<(f32, f32)>;
    fn set_exp_time_range(&self, min: f32, max: f32) -> XCamResult<()>;

    fn set_manual_exp(&self, gain: f32, time: f32) -> XCamResult<()>;

    fn set_blc_mode(&self, enabled: bool, mode: AeMeasAreaType) -> XCamResult<()>;
    fn set_blc_strength(&self, strength: i32) -> XCamResult<()>;
}

impl AutoExposure for Context {
    #[cfg(feature = "rel_1_0")]
    fn get_ae_mode(&self) -> XCamResult<AeMode> {
        unsafe {
            let mut mode = ffi::aeMode_t::default();
            XCamError::from(ffi::rk_aiq_uapi_getAeMode(
                self.internal.as_ptr(),
                &mut mode,
            ))
            .ok()
            .map(|_| mode)
        }
    }

    #[cfg(feature = "rel_1_0")]
    fn set_ae_mode(&self, mode: AeMode) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi_getAeMode(self.internal.as_ptr(), &mode)).ok() }
    }

    fn get_exp_mode(&self) -> XCamResult<OpMode> {
        unsafe {
            let mut mode = ffi::opMode_t::OP_INVAL;
            XCamError::from(ffi::rk_aiq_uapi_getExpMode(
                self.internal.as_ptr(),
                &mut mode,
            ))
            .ok()
            .map(|_| mode.into())
        }
    }

    fn set_exp_mode(&self, mode: OpMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setExpMode(
                self.internal.as_ptr(),
                mode.into(),
            ))
            .ok()
        }
    }

    fn get_exp_gain_range(&self) -> XCamResult<(f32, f32)> {
        unsafe {
            let mut range = ffi::paRange_t::default();
            XCamError::from(ffi::rk_aiq_uapi_getExpGainRange(
                self.internal.as_ptr(),
                &mut range,
            ))
            .ok()
            .map(|_| (range.min, range.max))
        }
    }

    fn set_exp_gain_range(&self, min: f32, max: f32) -> XCamResult<()> {
        let mut range = ffi::paRange_t { min, max };
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setExpGainRange(
                self.internal.as_ptr(),
                &mut range,
            ))
            .ok()
        }
    }

    fn get_exp_time_range(&self) -> XCamResult<(f32, f32)> {
        unsafe {
            let mut range = ffi::paRange_t::default();
            XCamError::from(ffi::rk_aiq_uapi_getExpTimeRange(
                self.internal.as_ptr(),
                &mut range,
            ))
            .ok()
            .map(|_| (range.min, range.max))
        }
    }

    fn set_exp_time_range(&self, min: f32, max: f32) -> XCamResult<()> {
        let mut range = ffi::paRange_t { min, max };
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setExpTimeRange(
                self.internal.as_ptr(),
                &mut range,
            ))
            .ok()
        }
    }

    fn set_manual_exp(&self, gain: f32, time: f32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setManualExp(
                self.internal.as_ptr(),
                gain,
                time,
            ))
            .ok()
        }
    }

    fn set_blc_mode(&self, enabled: bool, mode: AeMeasAreaType) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setBLCMode(
                self.internal.as_ptr(),
                enabled,
                mode.into(),
            ))
            .ok()
        }
    }

    fn set_blc_strength(&self, strength: i32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setBLCStrength(
                self.internal.as_ptr(),
                strength,
            ))
            .ok()
        }
    }
}

// pub enum ExpMode {
//     Auto,
//     Manual,
//     Invalid,
// }

// impl From<ffi::opMode_t> for ExpMode {
//     fn from(val: ffi::opMode_t) -> Self {
//         use ffi::opMode_t::*;
//         match val {
//             OP_AUTO => Self::Auto,
//             OP_MANUAL => Self::Manual,
//             OP_INVAL => Self::Invalid,
//             _ => Self::Invalid,
//         }
//     }
// }

// impl From<ExpMode> for ffi::opMode_t {
//     fn from(val: ExpMode) -> Self {
//         use ffi::opMode_t::*;
//         match val {
//             ExpMode::Auto => OP_AUTO,
//             ExpMode::Manual => OP_MANUAL,
//             ExpMode::Invalid => OP_INVAL,
//         }
//     }
// }
