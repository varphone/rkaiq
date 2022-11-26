//! 自动白平衡
//!
//! AWB 模块的功能是通过改变拍摄设备的色彩通道的增益，
//！对色温环境所造成的颜色偏差和拍摄设备本身所固有的色彩通道增益的偏差进行统一补偿，
//！从而让获得的图像能正确反映物体的真实色彩。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{OpMode, XCamResult};

pub trait NoiseRemoval {
    fn get_nr_mode(&self) -> XCamResult<OpMode>;
    fn set_nr_mode(&self, mode: OpMode) -> XCamResult<()>;

    fn get_anr_strength(&self) -> XCamResult<u32>;
    fn set_anr_strength(&self, strength: u32) -> XCamResult<()>;

    fn get_ms_nr_strength(&self) -> XCamResult<(bool, u32)>;
    fn set_ms_nr_strength(&self, on: bool, strength: u32) -> XCamResult<()>;

    fn get_mt_nr_strength(&self) -> XCamResult<(bool, u32)>;
    fn set_mt_nr_strength(&self, on: bool, strength: u32) -> XCamResult<()>;

    fn enable_bayernr2d(&self) -> XCamResult<()>;
    fn disable_bayernr2d(&self) -> XCamResult<()>;

    fn enable_bayernr3d(&self) -> XCamResult<()>;
    fn disable_bayernr3d(&self) -> XCamResult<()>;

    fn enable_cnr(&self) -> XCamResult<()>;
    fn disable_cnr(&self) -> XCamResult<()>;

    fn enable_ynr(&self) -> XCamResult<()>;
    fn disable_ynr(&self) -> XCamResult<()>;
}

impl NoiseRemoval for Context {
    fn get_nr_mode(&self) -> XCamResult<OpMode> {
        let mut mode = ffi::opMode_t::OP_INVAL;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getNRMode(
                self.internal.as_ptr(),
                &mut mode,
            ))
            .ok()
            .map(|_| mode.into())
        }
    }

    fn set_nr_mode(&self, mode: OpMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setNRMode(
                self.internal.as_ptr(),
                mode.into(),
            ))
            .ok()
        }
    }

    fn get_anr_strength(&self) -> XCamResult<u32> {
        let mut strength: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getANRStrth(
                self.internal.as_ptr(),
                &mut strength,
            ))
            .ok()
            .map(|_| strength)
        }
    }

    fn set_anr_strength(&self, strength: u32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setANRStrth(
                self.internal.as_ptr(),
                strength,
            ))
            .ok()
        }
    }

    fn get_ms_nr_strength(&self) -> XCamResult<(bool, u32)> {
        let mut on: bool = false;
        let mut strength: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getMSpaNRStrth(
                self.internal.as_ptr(),
                &mut on,
                &mut strength,
            ))
            .ok()
            .map(|_| (on, strength))
        }
    }

    fn set_ms_nr_strength(&self, on: bool, strength: u32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setMSpaNRStrth(
                self.internal.as_ptr(),
                on,
                strength,
            ))
            .ok()
        }
    }

    fn get_mt_nr_strength(&self) -> XCamResult<(bool, u32)> {
        let mut on: bool = false;
        let mut strength: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getMTNRStrth(
                self.internal.as_ptr(),
                &mut on,
                &mut strength,
            ))
            .ok()
            .map(|_| (on, strength))
        }
    }

    fn set_mt_nr_strength(&self, on: bool, strength: u32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_setMTNRStrth(
                self.internal.as_ptr(),
                on,
                strength,
            ))
            .ok()
        }
    }

    fn enable_bayernr2d(&self) -> XCamResult<()> {
        unsafe {
            let mut attr: ffi::rk_aiq_bayernr_attrib_v2_t = Default::default();
            XCamError::from(ffi::rk_aiq_user_api2_abayernrV2_GetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()?;
            match attr.eMode {
                ffi::Abayernr_OPMode_t::ABAYERNR_OP_MODE_AUTO => {
                    attr.stAuto.bayernr2DEn = 1;
                }
                ffi::Abayernr_OPMode_t::ABAYERNR_OP_MODE_MANUAL => {
                    attr.stManual.bayernr2DEn = 1;
                }
                _ => {}
            }
            XCamError::from(ffi::rk_aiq_user_api2_abayernrV2_SetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()
        }
    }

    fn disable_bayernr2d(&self) -> XCamResult<()> {
        unsafe {
            let mut attr: ffi::rk_aiq_bayernr_attrib_v2_t = Default::default();
            XCamError::from(ffi::rk_aiq_user_api2_abayernrV2_GetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()?;
            match attr.eMode {
                ffi::Abayernr_OPMode_t::ABAYERNR_OP_MODE_AUTO => {
                    attr.stAuto.bayernr2DEn = 0;
                }
                ffi::Abayernr_OPMode_t::ABAYERNR_OP_MODE_MANUAL => {
                    attr.stManual.bayernr2DEn = 0;
                }
                _ => {}
            }
            XCamError::from(ffi::rk_aiq_user_api2_abayernrV2_SetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()
        }
    }

    fn enable_bayernr3d(&self) -> XCamResult<()> {
        unsafe {
            let mut attr: ffi::rk_aiq_bayernr_attrib_v2_t = Default::default();
            XCamError::from(ffi::rk_aiq_user_api2_abayernrV2_GetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()?;
            match attr.eMode {
                ffi::Abayernr_OPMode_t::ABAYERNR_OP_MODE_AUTO => {
                    attr.stAuto.bayernr3DEn = 1;
                }
                ffi::Abayernr_OPMode_t::ABAYERNR_OP_MODE_MANUAL => {
                    attr.stManual.bayernr3DEn = 1;
                }
                _ => {}
            }
            XCamError::from(ffi::rk_aiq_user_api2_abayernrV2_SetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()
        }
    }

    fn disable_bayernr3d(&self) -> XCamResult<()> {
        unsafe {
            let mut attr: ffi::rk_aiq_bayernr_attrib_v2_t = Default::default();
            XCamError::from(ffi::rk_aiq_user_api2_abayernrV2_GetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()?;
            match attr.eMode {
                ffi::Abayernr_OPMode_t::ABAYERNR_OP_MODE_AUTO => {
                    attr.stAuto.bayernr3DEn = 0;
                }
                ffi::Abayernr_OPMode_t::ABAYERNR_OP_MODE_MANUAL => {
                    attr.stManual.bayernr3DEn = 0;
                }
                _ => {}
            }
            XCamError::from(ffi::rk_aiq_user_api2_abayernrV2_SetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()
        }
    }

    fn enable_cnr(&self) -> XCamResult<()> {
        unsafe {
            let mut attr: ffi::rk_aiq_cnr_attrib_v1_t = Default::default();
            XCamError::from(ffi::rk_aiq_user_api2_acnrV1_GetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()?;
            match attr.eMode {
                ffi::Acnr_OPMode_t::ACNR_OP_MODE_AUTO => {
                    attr.stAuto.cnrEn = 1;
                }
                ffi::Acnr_OPMode_t::ACNR_OP_MODE_MANUAL => {
                    attr.stManual.cnrEn = 1;
                }
                _ => {}
            }
            XCamError::from(ffi::rk_aiq_user_api2_acnrV1_SetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()
        }
    }

    fn disable_cnr(&self) -> XCamResult<()> {
        unsafe {
            let mut attr: ffi::rk_aiq_cnr_attrib_v1_t = Default::default();
            XCamError::from(ffi::rk_aiq_user_api2_acnrV1_GetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()?;
            match attr.eMode {
                ffi::Acnr_OPMode_t::ACNR_OP_MODE_AUTO => {
                    attr.stAuto.cnrEn = 0;
                }
                ffi::Acnr_OPMode_t::ACNR_OP_MODE_MANUAL => {
                    attr.stManual.cnrEn = 0;
                }
                _ => {}
            }
            XCamError::from(ffi::rk_aiq_user_api2_acnrV1_SetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()
        }
    }

    fn enable_ynr(&self) -> XCamResult<()> {
        unsafe {
            let mut attr: ffi::rk_aiq_ynr_attrib_v2_t = Default::default();
            XCamError::from(ffi::rk_aiq_user_api2_aynrV2_GetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()?;
            match attr.eMode {
                ffi::Aynr_OPMode_t::AYNR_OP_MODE_AUTO => {
                    attr.stAuto.ynrEn = 1;
                }
                ffi::Aynr_OPMode_t::AYNR_OP_MODE_MANUAL => {
                    attr.stManual.ynrEn = 1;
                }
                _ => {}
            }
            XCamError::from(ffi::rk_aiq_user_api2_aynrV2_SetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()
        }
    }

    fn disable_ynr(&self) -> XCamResult<()> {
        unsafe {
            let mut attr: ffi::rk_aiq_ynr_attrib_v2_t = Default::default();
            XCamError::from(ffi::rk_aiq_user_api2_aynrV2_GetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()?;
            match attr.eMode {
                ffi::Aynr_OPMode_t::AYNR_OP_MODE_AUTO => {
                    attr.stAuto.ynrEn = 0;
                }
                ffi::Aynr_OPMode_t::AYNR_OP_MODE_MANUAL => {
                    attr.stManual.ynrEn = 0;
                }
                _ => {}
            }
            XCamError::from(ffi::rk_aiq_user_api2_aynrV2_SetAttrib(
                self.internal.as_ptr(),
                &mut attr,
            ))
            .ok()
        }
    }
}
