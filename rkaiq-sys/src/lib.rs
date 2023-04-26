//! RKAIQ ISP 应用接口安全绑定。
//!
//! 本项目当前基于 RKAIQ V1.0 ISP 应用接口接口实现。
//!
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::derivable_impls)]
use std::ffi::CStr;
use std::fmt::{self, Debug};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for antiFlickerMode_t {
    fn default() -> Self {
        antiFlickerMode_t::ANTIFLICKER_NORMAL_MODE
    }
}

impl Default for expPwrLineFreq_t {
    fn default() -> Self {
        expPwrLineFreq_t::EXP_PWR_LINE_FREQ_DIS
    }
}

impl Default for opMode_t {
    fn default() -> Self {
        opMode_t::OP_INVAL
    }
}

impl Default for rk_aiq_gray_mode_t {
    fn default() -> Self {
        rk_aiq_gray_mode_t::RK_AIQ_GRAY_MODE_CPSL
    }
}

impl Default for rk_aiq_wb_scene_t {
    fn default() -> Self {
        rk_aiq_wb_scene_t::RK_AIQ_WBCT_INCANDESCENT
    }
}

impl fmt::Debug for rk_aiq_lens_info_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len_name = unsafe { CStr::from_ptr(self.len_name.as_ptr()) };
        f.debug_struct("rk_aiq_lens_info_t")
            .field("len_name", &len_name)
            .finish()
    }
}

impl fmt::Debug for rk_aiq_sensor_info_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sensor_name = unsafe { CStr::from_ptr(self.sensor_name.as_ptr()) };
        let mut ds = f.debug_struct("rk_aiq_sensor_info_t");
        ds.field("sensor_name", &sensor_name)
            .field("support_fmt", &self.support_fmt)
            .field("num", &self.num)
            .field("binded_strm_media_idx", &self.binded_strm_media_idx);
        #[cfg(feature = "v3_0")]
        ds.field("phyId", &self.phyId);
        ds.finish()
    }
}

impl rk_aiq_static_info_t {
    pub fn sensor_name(&self) -> String {
        unsafe {
            std::ffi::CStr::from_ptr(self.sensor_info.sensor_name.as_ptr())
                .to_string_lossy()
                .into()
        }
    }
}

impl fmt::Debug for rk_aiq_static_info_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("rk_aiq_static_info_t");
        ds.field("sensor_info", &self.sensor_info)
            .field("lens_info", &self.lens_info)
            .field("isp_hw_ver", &self.isp_hw_ver)
            .field("has_lens_vcm", &self.has_lens_vcm)
            .field("has_fl", &self.has_fl)
            .field("fl_strth_adj_sup", &self.fl_strth_adj_sup)
            .field("has_irc", &self.has_irc)
            .field("fl_ir_strth_adj_sup", &self.fl_ir_strth_adj_sup);
        #[cfg(feature = "v3_0")]
        ds.field("_is_1608_sensor", &self._is_1608_sensor)
            .field("is_multi_isp_mode", &self.is_multi_isp_mode)
            .field("multi_isp_extended_pixel", &self.multi_isp_extended_pixel)
            .field("reserved", &self.reserved);
        ds.finish()
    }
}
