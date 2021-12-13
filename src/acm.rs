use super::context::Context;
use super::error::XCamError;
use super::ffi;
use super::types::XCamResult;

pub trait AutoColorManagment {
    /// 获取亮度等级。
    fn get_brightness(&self) -> XCamResult<u32>;

    /// 设置亮度等级，范围：[0,255]。
    fn set_brightness(&self, val: u32) -> XCamResult<()>;

    /// 获取对比度等级。
    fn get_contrast(&self) -> XCamResult<u32>;

    /// 设置对比度等级，范围：[0,255]。
    fn set_contrast(&self, val: u32) -> XCamResult<()>;

    /// 获取饱和度等级。
    fn get_saturation(&self) -> XCamResult<u32>;

    /// 设置饱和度等级，范围：[0,255]。
    fn set_saturation(&self, val: u32) -> XCamResult<()>;

    /// 获取色度等级。
    fn get_hue(&self) -> XCamResult<u32>;

    /// 设置色度等级，范围：[0,255]。
    fn set_hue(&self, val: u32) -> XCamResult<()>;
}

impl AutoColorManagment for Context {
    fn get_brightness(&self) -> XCamResult<u32> {
        let mut val: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getBrightness(
                self.internal.as_ptr(),
                &mut val,
            ))
            .ok()
            .map(|_| val)
        }
    }
    fn set_brightness(&self, val: u32) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi_setBrightness(self.internal.as_ptr(), val)).ok() }
    }

    fn get_contrast(&self) -> XCamResult<u32> {
        let mut val: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getContrast(
                self.internal.as_ptr(),
                &mut val,
            ))
            .ok()
            .map(|_| val)
        }
    }

    fn set_contrast(&self, val: u32) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi_setContrast(self.internal.as_ptr(), val)).ok() }
    }

    fn get_saturation(&self) -> XCamResult<u32> {
        let mut val: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getSaturation(
                self.internal.as_ptr(),
                &mut val,
            ))
            .ok()
            .map(|_| val)
        }
    }
    fn set_saturation(&self, val: u32) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi_setSaturation(self.internal.as_ptr(), val)).ok() }
    }

    fn get_hue(&self) -> XCamResult<u32> {
        let mut val: u32 = 0;
        unsafe {
            XCamError::from(ffi::rk_aiq_uapi_getHue(self.internal.as_ptr(), &mut val))
                .ok()
                .map(|_| val)
        }
    }

    fn set_hue(&self, val: u32) -> XCamResult<()> {
        unsafe { XCamError::from(ffi::rk_aiq_uapi_setHue(self.internal.as_ptr(), val)).ok() }
    }
}
