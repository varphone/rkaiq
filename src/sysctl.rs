//! 系统控制。
//!
//! 系统控制部分包含了 AIQ 公共属性配置，初始化 AIQ、运行 AIQ、退出AIQ，设置 AIQ 各模块等功能。
use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::{
    AlgoContext, AlgoDescComm, ModuleId, Rect, StaticInfo, WorkingMode, XCamResult,
};
use std::ffi::{CStr, CString};

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

    fn update_iq<T: Into<Vec<u8>>>(&self, iq_file: T) -> XCamResult<()>;

    fn get_crop(&self) -> XCamResult<Rect>;

    fn set_crop(&self, crop: Rect) -> XCamResult<()>;
}

impl SystemControl for Context {
    fn prepare(&self, width: u32, height: u32, mode: WorkingMode) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_sysctl_prepare(
                self.internal.as_ptr(),
                width,
                height,
                mode.into(),
            ))
            .ok()
        }
    }

    fn start(&self) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi_sysctl_start(self.internal.as_ptr())).ok() }
    }

    fn stop(&self, keep_ext_hw_st: bool) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_sysctl_stop(
                self.internal.as_ptr(),
                keep_ext_hw_st,
            ))
            .ok()
        }
    }

    fn enable_module<T: Into<ModuleId>>(&self, id: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_sysctl_setModuleCtl(
                self.internal.as_ptr(),
                id.into(),
                true,
            ))
            .ok()
        }
    }

    fn disable_module<T: Into<ModuleId>>(&self, id: T) -> XCamResult<()> {
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_sysctl_setModuleCtl(
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
            XCamError::from(ffi::rk_aiq_uapi_sysctl_getModuleCtl(
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
            XCamError::from(ffi::rk_aiq_uapi_sysctl_enableAxlib(
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
            XCamError::from(ffi::rk_aiq_uapi_sysctl_enableAxlib(
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
        ffi::rk_aiq_uapi_sysctl_getEnabledAxlibCtx(self.internal.as_ptr(), algo_type)
    }

    fn update_iq<T: Into<Vec<u8>>>(&self, iq_file: T) -> XCamResult<()> {
        let iq_file = CString::new(iq_file).unwrap();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_sysctl_updateIq(
                self.internal.as_ptr(),
                iq_file.as_ptr() as *mut _,
            ))
            .ok()
        }
    }

    fn get_crop(&self) -> XCamResult<Rect> {
        let mut crop = Rect::default();
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_sysctl_getCrop(
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
        let ptr = ffi::rk_aiq_uapi_sysctl_getBindedSnsEntNmByVd(vd.as_ptr());
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
        XCamError::from(ffi::rk_aiq_uapi_sysctl_getStaticMetas(
            vd.as_ptr(),
            &mut data,
        ))
        .ok()
        .map(|_| data)
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
