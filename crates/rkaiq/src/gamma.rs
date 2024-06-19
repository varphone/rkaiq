//! Gamma
//!
//! Gamma 模块对图像进行亮度空间非线性转换以适配输出设备。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
#[cfg(all(
    any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
    feature = "isp_hw_v21"
))]
use super::types::GammaApiManualV21;
#[cfg(all(
    any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
    feature = "isp_hw_v30"
))]
use super::types::GammaApiManualV30;
#[cfg(any(
    feature = "v2_0",
    all(
        any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
        feature = "isp_hw_v21"
    )
))]
use super::types::GammaCurveType;
#[cfg(feature = "v2_0")]
use super::types::{GammaApiManual, GammaCurveUsrDefine1Para, GammaCurveUsrDefine2Para};
use super::types::{GammaAttr, GammaCaliDb, GammaMode, XCamResult};

/// 一个描述 Gamma 控制的契定。
pub trait Gamma {
    /// 获取伽玛。
    fn get_gamma_coef(&self) -> XCamResult<GammaAttr>;

    /// 设置伽玛。
    fn set_gamma_coef<T: Into<GammaAttr>>(&self, gamma_attr: T) -> XCamResult<()>;

    /// 快速设置伽玛曲线。
    ///
    /// # Arguments
    ///
    /// * `gamma_coef` Gamma 系数，取值范围 [0,100]，默认值 2.2，精度 0.01。
    /// * `slope_at_zero` 暗区斜率，取值范围 [-0.05,0.05]，默认值 0，精度 0.001。
    fn set_gamma_coef_fast(&self, gamma_coef: f32, slope_at_zero: f32) -> XCamResult<()>;
}

impl Gamma for Context {
    fn get_gamma_coef(&self) -> XCamResult<GammaAttr> {
        #[cfg(feature = "v2_0")]
        unsafe {
            let mut gamma_attr = GammaAttr::default();
            XCamError::from(ffi::rk_aiq_user_api_agamma_GetAttrib(
                self.internal.as_ptr(),
                &mut gamma_attr,
            ))
            .ok()
            .map(|_| gamma_attr)
        }
        #[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
        unsafe {
            let mut gamma_attr = GammaAttr::default();
            XCamError::from(ffi::rk_aiq_user_api2_agamma_GetAttrib(
                self.internal.as_ptr(),
                &mut gamma_attr,
            ))
            .ok()
            .map(|_| gamma_attr)
        }
    }

    fn set_gamma_coef<T: Into<GammaAttr>>(&self, gamma_attr: T) -> XCamResult<()> {
        #[cfg(feature = "v2_0")]
        unsafe {
            XCamError::from(ffi::rk_aiq_user_api_agamma_SetAttrib(
                self.internal.as_ptr(),
                gamma_attr.into(),
            ))
            .ok()
        }
        #[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
        unsafe {
            XCamError::from(ffi::rk_aiq_user_api2_agamma_SetAttrib(
                self.internal.as_ptr(),
                gamma_attr.into(),
            ))
            .ok()
        }
    }

    fn set_gamma_coef_fast(&self, gamma_coef: f32, slope_at_zero: f32) -> XCamResult<()> {
        #[cfg(feature = "v2_0")]
        unsafe {
            XCamError::from(-1).ok()
        }
        #[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_setGammaCoef(
                self.internal.as_ptr(),
                gamma_coef,
                slope_at_zero,
            ))
            .ok()
        }
    }
}

/// 一个代表 Gamma 属性构建器的类型。
#[allow(dead_code)]
pub struct GammaAttrBuilder {
    mode: Option<GammaMode>,
    #[cfg(feature = "v2_0")]
    manual: Option<GammaApiManual>,
    #[cfg(all(
        any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
        feature = "isp_hw_v21"
    ))]
    manual: Option<GammaApiManualV21>,
    #[cfg(all(
        any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
        feature = "isp_hw_v30"
    ))]
    manual: Option<GammaApiManualV30>,
    tool: Option<GammaCaliDb>,
    scene_mode: Option<i32>,
}

impl GammaAttrBuilder {
    /// 创建一个 Gamma 属性构建器实例。
    pub fn new() -> Self {
        Self {
            mode: None,
            #[cfg(feature = "v2_0")]
            manual: None,
            #[cfg(any(feature = "v3_0", feature = "v4_0", feature = "v5_0"))]
            manual: None,
            tool: None,
            scene_mode: None,
        }
    }

    /// 创建一个用于配置手动曲线系数的 Gamma 属性构建器。
    #[cfg(feature = "v2_0")]
    pub fn with_manual_usr_define1(coef1: f32, coef2: f32) -> Self {
        Self {
            mode: Some(GammaMode::RK_AIQ_GAMMA_MODE_MANUAL),
            manual: Some(GammaApiManual {
                en: true,
                CurveType: GammaCurveType::RK_GAMMA_CURVE_TYPE_USER_DEFINE1,
                user1: GammaCurveUsrDefine1Para { coef1, coef2 },
                user2: Default::default(),
            }),
            tool: None,
            scene_mode: None,
        }
    }

    /// 创建一个用于配置手动曲线系数的 Gamma 属性构建器。
    #[cfg(all(
        any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
        feature = "isp_hw_v21"
    ))]
    pub fn with_manual_usr_define1(_coef1: f32, _coef2: f32) -> Self {
        Self {
            mode: Some(GammaMode::GAMMA_MODE_MANUAL),
            manual: Some(GammaApiManualV21 {
                Gamma_en: true,
                Gamma_out_segnum: GammaCurveType::GAMMATYPE_LOG,
                Gamma_out_offset: 0,
                Gamma_curve: [0; 45],
            }),
            tool: None,
            scene_mode: None,
        }
    }

    /// 创建一个用于配置手动曲线系数的 Gamma 属性构建器。
    #[cfg(all(
        any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
        feature = "isp_hw_v30"
    ))]
    pub fn with_manual_usr_define1(_coef1: f32, _coef2: f32) -> Self {
        Self {
            mode: Some(GammaMode::GAMMA_MODE_MANUAL),
            manual: Some(GammaApiManualV30 {
                Gamma_en: true,
                Gamma_out_offset: 0,
                Gamma_curve: [0; 49],
            }),
            tool: None,
            scene_mode: None,
        }
    }

    /// 创建一个用于配置手动曲线表的 Gamma 属性构建器。
    #[cfg(feature = "v2_0")]
    pub fn with_manual_usr_define2(table: &[i32]) -> Self {
        assert_eq!(table.len(), 45);
        let mut gamma_table: [i32; 45] = [0; 45];
        gamma_table.copy_from_slice(&table[0..45]);
        Self {
            mode: Some(GammaMode::RK_AIQ_GAMMA_MODE_MANUAL),
            manual: Some(GammaApiManual {
                en: true,
                CurveType: GammaCurveType::RK_GAMMA_CURVE_TYPE_USER_DEFINE2,
                user1: Default::default(),
                user2: GammaCurveUsrDefine2Para {
                    gamma_out_segnum: 0,
                    gamma_out_offset: 0,
                    gamma_table,
                },
            }),
            tool: None,
            scene_mode: None,
        }
    }

    /// 创建一个用于配置手动曲线表的 Gamma 属性构建器。
    #[cfg(all(
        any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
        feature = "isp_hw_v21"
    ))]
    pub fn with_manual_usr_define2(table: &[u16]) -> Self {
        assert_eq!(table.len(), 45);
        let mut gamma_table: [u16; 45] = [0; 45];
        gamma_table.copy_from_slice(&table[0..45]);
        Self {
            mode: Some(GammaMode::GAMMA_MODE_MANUAL),
            manual: Some(GammaApiManualV21 {
                Gamma_en: true,
                Gamma_out_segnum: GammaCurveType::GAMMATYPE_LOG,
                Gamma_out_offset: 0,
                Gamma_curve: gamma_table,
            }),
            tool: None,
            scene_mode: None,
        }
    }

    /// 创建一个用于配置手动曲线表的 Gamma 属性构建器。
    #[cfg(all(
        any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
        feature = "isp_hw_v30"
    ))]
    pub fn with_manual_usr_define2(table: &[u16]) -> Self {
        assert_eq!(table.len(), 49);
        let mut gamma_table: [u16; 49] = [0; 49];
        gamma_table.copy_from_slice(&table[0..49]);
        Self {
            mode: Some(GammaMode::GAMMA_MODE_MANUAL),
            manual: Some(GammaApiManualV30 {
                Gamma_en: true,
                Gamma_out_offset: 0,
                Gamma_curve: gamma_table,
            }),
            tool: None,
            scene_mode: None,
        }
    }

    /// 返回 Gamma 属性。
    pub fn build(self) -> GammaAttr {
        let mut attr = GammaAttr::default();
        #[cfg(feature = "v2_0")]
        if let Some(ref v) = self.mode {
            attr.mode = *v;
        }
        #[cfg(all(
            any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
            feature = "isp_hw_v21"
        ))]
        if let Some(ref v) = self.mode {
            attr.atrrV21.mode = *v;
        }
        #[cfg(all(
            any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
            feature = "isp_hw_v30"
        ))]
        if let Some(ref v) = self.mode {
            attr.atrrV30.mode = *v;
        }
        #[cfg(feature = "v2_0")]
        if let Some(ref v) = self.manual {
            attr.stManual = *v;
        }
        #[cfg(all(
            any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
            feature = "isp_hw_v21"
        ))]
        if let Some(ref v) = self.manual {
            attr.atrrV21.stManual = *v;
        }
        #[cfg(all(
            any(feature = "v3_0", feature = "v4_0", feature = "v5_0"),
            feature = "isp_hw_v30"
        ))]
        if let Some(ref v) = self.manual {
            attr.atrrV30.stManual = *v;
        }
        #[cfg(feature = "v2_0")]
        if let Some(ref v) = self.tool {
            attr.stTool = *v;
        }
        #[cfg(feature = "v2_0")]
        if let Some(ref v) = self.scene_mode {
            attr.Scene_mode = *v;
        }
        attr
    }
}

impl Default for GammaAttrBuilder {
    fn default() -> Self {
        Self::new()
    }
}
