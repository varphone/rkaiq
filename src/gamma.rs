//! Gamma
//!
//! Gamma 模块对图像进行亮度空间非线性转换以适配输出设备。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{
    GammaApiManual, GammaAttr, GammaCaliDb, GammaCurveType, GammaCurveUsrDefine1Para,
    GammaCurveUsrDefine2Para, GammaOpMode, XCamResult,
};

/// 一个描述 Gamma 控制的契定。
pub trait Gamma {
    /// 获取伽玛。
    fn get_gamma_coef(&self) -> XCamResult<GammaAttr>;

    /// 设置伽玛。
    fn set_gamma_coef<T: Into<GammaAttr>>(&self, gamma_attr: T) -> XCamResult<()>;
}

impl Gamma for Context {
    fn get_gamma_coef(&self) -> XCamResult<GammaAttr> {
        unsafe {
            let mut gamma_attr = GammaAttr::default();
            XCamError::from(ffi::rk_aiq_user_api_agamma_GetAttrib(
                self.internal.as_ptr(),
                &mut gamma_attr,
            ))
            .ok()
            .map(|_| gamma_attr)
        }
    }

    fn set_gamma_coef<T: Into<GammaAttr>>(&self, gamma_attr: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_user_api_agamma_SetAttrib(
                self.internal.as_ptr(),
                gamma_attr.into(),
            ))
            .ok()
        }
    }
}

/// 一个代表 Gamma 属性构建器的类型。
pub struct GammaAttrBuilder {
    mode: Option<GammaOpMode>,
    manual: Option<GammaApiManual>,
    tool: Option<GammaCaliDb>,
    scene_mode: Option<i32>,
}

impl GammaAttrBuilder {
    /// 创建一个 Gamma 属性构建器实例。
    pub fn new() -> Self {
        Self {
            mode: None,
            manual: None,
            tool: None,
            scene_mode: None,
        }
    }

    /// 创建一个用于配置手动曲线系数的 Gamma 属性构建器。
    pub fn with_manual_usr_define1(coef1: f32, coef2: f32) -> Self {
        Self {
            mode: Some(GammaOpMode::RK_AIQ_GAMMA_MODE_MANUAL),
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

    /// 创建一个用于配置手动曲线表的 Gamma 属性构建器。
    pub fn with_manual_usr_define2(table: &[i32]) -> Self {
        assert_eq!(table.len(), 45);
        let mut gamma_table: [i32; 45] = [0; 45];
        gamma_table.copy_from_slice(&table[0..45]);
        Self {
            mode: Some(GammaOpMode::RK_AIQ_GAMMA_MODE_MANUAL),
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

    /// 返回 Gamma 属性。
    pub fn build(self) -> GammaAttr {
        let mut attr = GammaAttr::default();
        if let Some(ref v) = self.mode {
            attr.mode = *v;
        }
        if let Some(ref v) = self.manual {
            attr.stManual = *v;
        }
        if let Some(ref v) = self.tool {
            attr.stTool = *v;
        }
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
