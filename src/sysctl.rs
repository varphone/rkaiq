//! 系统控制。
//!
//! 系统控制部分包含了 AIQ 公共属性配置，初始化 AIQ、运行 AIQ、退出AIQ，设置 AIQ 各模块等功能。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{
    AlgoContext, AlgoDescComm, CpslCap, CpslCfg, CpslInfo, ModuleId, Rect, StaticInfo, WorkingMode,
    XCamResult,
};
use std::ffi::{CStr, CString};

/// 一个描述静态信息枚举器的类型。
pub struct StaticMetas {
    index: i32,
}

impl StaticMetas {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Default for StaticMetas {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for StaticMetas {
    type Item = ffi::rk_aiq_static_info_t;

    fn next(&mut self) -> Option<Self::Item> {
        enum_static_metas(self.index)
            .map(|x| {
                self.index += 1;
                x
            })
            .ok()
    }
}

pub trait SystemControl {
    fn prepare(&self, width: u32, height: u32, mode: WorkingMode) -> XCamResult<()>;

    fn start(&self) -> XCamResult<()>;

    fn stop(&self, keep_ext_hw_st: bool) -> XCamResult<()>;

    fn enable_module<T: Into<ModuleId>>(&self, id: T) -> XCamResult<()>;

    fn disable_module<T: Into<ModuleId>>(&self, id: T) -> XCamResult<()>;

    fn is_module_enabled<T: Into<ModuleId>>(&self, id: T) -> bool;

    fn register_lib(&self, algo_lib_des: AlgoDescComm) -> XCamResult<()>;

    fn unregister_lib(&self, algo_type: i32, lib_id: i32) -> XCamResult<()>;

    fn enable_ax_lib(&self, algo_type: i32, lib_id: i32) -> XCamResult<()>;

    fn disable_ax_lib(&self, algo_type: i32, lib_id: i32) -> XCamResult<()>;

    fn is_ax_lib_enabled(&self, algo_type: i32, lib_id: i32) -> bool;

    /// 获取使能算法库的上下文结构体。
    ///
    /// # Safety
    /// 请确保返回值仅在 Context 生存期间使用。
    unsafe fn get_enabled_ax_lib_ctx(&self, algo_type: i32) -> *const AlgoContext;

    /// 获取补光灯控制信息。
    fn get_cps_lt_info(&self) -> XCamResult<CpslInfo>;

    /// 查询补光灯的支持能力。
    fn query_cps_lt_cap(&self) -> XCamResult<CpslCap>;

    /// 设置补光灯控制信息。
    fn set_cps_lt_cfg<T: Into<CpslCfg>>(&self, cfg: T) -> XCamResult<()>;

    fn update_iq<T: Into<Vec<u8>>>(&self, iq_file: T) -> XCamResult<()>;

    fn get_crop(&self) -> XCamResult<Rect>;

    fn set_crop(&self, crop: Rect) -> XCamResult<()>;
}

impl SystemControl for Context {
    fn prepare(&self, width: u32, height: u32, mode: WorkingMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_prepare(
                self.internal.as_ptr(),
                width,
                height,
                mode.into(),
            ))
            .ok()
        }
    }

    fn start(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi2_sysctl_start(self.internal.as_ptr())).ok() }
    }

    fn stop(&self, keep_ext_hw_st: bool) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_stop(
                self.internal.as_ptr(),
                keep_ext_hw_st,
            ))
            .ok()
        }
    }

    fn enable_module<T: Into<ModuleId>>(&self, id: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_setModuleCtl(
                self.internal.as_ptr(),
                id.into(),
                true,
            ))
            .ok()
        }
    }

    fn disable_module<T: Into<ModuleId>>(&self, id: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_setModuleCtl(
                self.internal.as_ptr(),
                id.into(),
                false,
            ))
            .ok()
        }
    }

    fn is_module_enabled<T: Into<ModuleId>>(&self, id: T) -> bool {
        let mut enabled = false;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_getModuleCtl(
                self.internal.as_ptr(),
                id.into(),
                &mut enabled,
            ))
            .ok()
            .map_or(false, |_| enabled)
        }
    }

    fn register_lib(&self, mut algo_lib_des: AlgoDescComm) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_sysctl_regLib(
                self.internal.as_ptr(),
                &mut algo_lib_des,
            ))
            .ok()
        }
    }

    fn unregister_lib(&self, algo_type: i32, lib_id: i32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_sysctl_unRegLib(
                self.internal.as_ptr(),
                algo_type,
                lib_id,
            ))
            .ok()
        }
    }

    fn enable_ax_lib(&self, algo_type: i32, lib_id: i32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_enableAxlib(
                self.internal.as_ptr(),
                algo_type,
                lib_id,
                true,
            ))
            .ok()
        }
    }

    fn disable_ax_lib(&self, algo_type: i32, lib_id: i32) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_enableAxlib(
                self.internal.as_ptr(),
                algo_type,
                lib_id,
                false,
            ))
            .ok()
        }
    }

    fn is_ax_lib_enabled(&self, algo_type: i32, lib_id: i32) -> bool {
        unsafe { ffi::rk_aiq_uapi_sysctl_getAxlibStatus(self.internal.as_ptr(), algo_type, lib_id) }
    }

    unsafe fn get_enabled_ax_lib_ctx(&self, algo_type: i32) -> *const AlgoContext {
        ffi::rk_aiq_uapi2_sysctl_getEnabledAxlibCtx(self.internal.as_ptr(), algo_type)
    }

    fn get_cps_lt_info(&self) -> XCamResult<CpslInfo> {
        let mut info = CpslInfo::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_getCpsLtInfo(
                self.internal.as_ptr(),
                &mut info,
            ))
            .ok()
            .map(|_| info)
        }
    }

    fn query_cps_lt_cap(&self) -> XCamResult<CpslCap> {
        let mut cap = CpslCap::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_queryCpsLtCap(
                self.internal.as_ptr(),
                &mut cap,
            ))
            .ok()
            .map(|_| cap)
        }
    }

    fn set_cps_lt_cfg<T: Into<CpslCfg>>(&self, cfg: T) -> XCamResult<()> {
        let mut cfg = cfg.into();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_setCpsLtCfg(
                self.internal.as_ptr(),
                &mut cfg,
            ))
            .ok()
        }
    }

    fn update_iq<T: Into<Vec<u8>>>(&self, iq_file: T) -> XCamResult<()> {
        let iq_file = CString::new(iq_file).unwrap();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_updateIq(
                self.internal.as_ptr(),
                iq_file.as_ptr() as *mut _,
            ))
            .ok()
        }
    }

    fn get_crop(&self) -> XCamResult<Rect> {
        let mut crop = Rect::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi2_sysctl_getCrop(
                self.internal.as_ptr(),
                &mut crop,
            ))
            .ok()
            .map(|_| crop)
        }
    }

    fn set_crop(&self, crop: Rect) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_sysctl_setCrop(
                self.internal.as_ptr(),
                crop,
            ))
            .ok()
        }
    }
}

/// 查询 video 结点所对应的 sensor entity name。
///
/// # Note
///
/// 参数必须为 ISPP scale 结点路径。
pub fn get_binded_sensor_entity_name<T: Into<Vec<u8>>>(vd: T) -> Option<String> {
    let vd = CString::new(vd).expect("CString::new failed");
    unsafe {
        let ptr = ffi::rk_aiq_uapi2_sysctl_getBindedSnsEntNmByVd(vd.as_ptr());
        if ptr.is_null() {
            CStr::from_ptr(ptr).to_str().map(ToString::to_string).ok()
        } else {
            None
        }
    }
}

pub fn get_static_metas<T: Into<Vec<u8>>>(vd: T) -> XCamResult<StaticInfo> {
    let vd = CString::new(vd).expect("CString::new failed");
    unsafe {
        let mut data = ffi::rk_aiq_static_info_t::default();
        XCamError::from(ffi::rk_aiq_uapi2_sysctl_getStaticMetas(
            vd.as_ptr(),
            &mut data,
        ))
        .ok()
        .map(|_| data)
    }
}

/// 枚举 AIQ 获取到的静态信息。
///
/// # Parameters
/// * `index` - 索引号，从 0 开始。
pub fn enum_static_metas(index: i32) -> XCamResult<StaticInfo> {
    unsafe {
        let mut data = ffi::rk_aiq_static_info_t::default();
        XCamError::from(ffi::rk_aiq_uapi2_sysctl_enumStaticMetas(index, &mut data))
            .ok()
            .map(|_| data)
    }
}

/// 预先初始化 AIQ 系统配置。
pub fn pre_init(sns_ent_name: &str, mode: WorkingMode, iq_file: &str) -> XCamResult<()> {
    let sns = CString::new(sns_ent_name).expect("CString::new failed");
    let iq = CString::new(iq_file).expect("CString::new failed");
    unsafe {
        XCamError::from(ffi::rk_aiq_uapi2_sysctl_preInit(
            sns.as_ptr(),
            mode.into(),
            iq.as_ptr(),
        ))
        .ok()
    }
}

/// 设置全局日志等级。
#[cfg(feature = "fullv")]
pub fn set_gll(level: i32) {
    unsafe {
        #[cfg(any(feature = "v2_0", feature = "v3_0"))]
        ffi::rk_aiq_uapi_sysctl_set_gll(level);
        #[cfg(any(feature = "v4_0", feature = "v5_0"))]
        ffi::rk_aiq_uapi2_sysctl_set_gll(level);
    }
}

/// 获取全局日志等级。
#[cfg(feature = "fullv")]
pub fn get_gll() -> i32 {
    #[cfg(any(feature = "v2_0", feature = "v3_0"))]
    unsafe { ffi::rk_aiq_uapi_sysctl_get_gll() }
    #[cfg(any(feature = "v4_0", feature = "v5_0"))]
    unsafe { ffi::rk_aiq_uapi2_sysctl_get_gll() }
}

/// 初始化 RKAIQ 库。
#[cfg(feature = "fullv")]
pub fn init_lib() {
    unsafe {
        ffi::rk_aiq_init_lib();
    }
}

/// 释放 RKAIQ 库。
#[cfg(feature = "fullv")]
pub fn deinit_lib() {
    unsafe {
        ffi::rk_aiq_deinit_lib();
    }
}

/// 释放 RKAIQ 库。
#[cfg(feature = "fullv")]
pub fn set_log_callback(
    cb: Option<unsafe extern "C" fn(i32, *const std::os::raw::c_char, *const std::os::raw::c_char)>,
) {
    unsafe {
        #[cfg(any(feature = "v2_0", feature = "v3_0"))]
        ffi::rk_aiq_set_log_callback(cb);
        #[cfg(any(feature = "v4_0", feature = "v5_0"))]
        ffi::rk_aiq_uapi2_set_log_callback(cb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let r = Context::new("m00_b_ov5695 4-0036-1", "/etc/iqfiles");
        assert_eq!(r.is_ok(), true);
        if let Ok(ctx) = r {
            assert_eq!(ctx.prepare(2592, 1944, WorkingMode::Normal), Ok(()));
            assert_eq!(ctx.start(), Ok(()));
            assert_eq!(ctx.stop(false), Ok(()));
        }
    }

    #[test]
    fn test_get_binded_sensor_entity_name() {
        let r = get_binded_sensor_entity_name("/dev/video0");
        assert_eq!(r, None);
    }

    #[test]
    fn test_get_static_metas() {
        let r = get_static_metas("m00_b_ov5695 4-0036-1");
        assert_eq!(r.is_ok(), true);
        println!("{:?}", r.unwrap());
    }
}
