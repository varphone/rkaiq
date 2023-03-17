//! RKAIQ ISP 应用接口安全绑定。
//!
//! 本项目当前基于 RKAIQ V1.0 ISP 应用接口接口实现。
//!
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unaligned_references)]
#![allow(clippy::derivable_impls)]

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

impl rk_aiq_static_info_t {
    pub fn sensor_name(&self) -> String {
        unsafe {
            std::ffi::CStr::from_ptr(self.sensor_info.sensor_name.as_ptr())
                .to_string_lossy()
                .into()
        }
    }
}
