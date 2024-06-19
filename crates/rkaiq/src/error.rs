use super::ffi;
use std::fmt;

/// 一个描述摄像头访问错误代码的类型。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct XCamError(ffi::XCamReturn);

impl XCamError {
    pub fn ok(self) -> Result<(), Self> {
        if self.0 != ffi::XCamReturn::XCAM_RETURN_NO_ERROR {
            Err(self)
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for XCamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ffi::XCamReturn::*;
        let desc = match self.0 {
            XCAM_RETURN_NO_ERROR => "No Error",
            XCAM_RETURN_BYPASS => "ByPass",
            XCAM_RETURN_ERROR_FAILED => "Failed",
            XCAM_RETURN_ERROR_PARAM => "Bad Parameters",
            XCAM_RETURN_ERROR_MEM => "No Memory",
            XCAM_RETURN_ERROR_FILE => "File Error",
            XCAM_RETURN_ERROR_ANALYZER => "Analyzer Error",
            XCAM_RETURN_ERROR_ISP => "ISP Error",
            XCAM_RETURN_ERROR_SENSOR => "Sensor Error",
            XCAM_RETURN_ERROR_THREAD => "Thread Error",
            XCAM_RETURN_ERROR_IOCTL => "IOCTL Error",
            XCAM_RETURN_ERROR_ORDER => "Wrong Order",
            XCAM_RETURN_ERROR_TIMEOUT => "Timeout",
            XCAM_RETURN_ERROR_OUTOFRANGE => "Out of Range",
            _ => "Unknown",
        };
        write!(f, "{}", desc)
    }
}

impl From<i32> for XCamError {
    fn from(val: i32) -> Self {
        use ffi::XCamReturn::*;
        let val = match val {
            0 => XCAM_RETURN_NO_ERROR,
            1 => XCAM_RETURN_BYPASS,
            -1 => XCAM_RETURN_ERROR_FAILED,
            -2 => XCAM_RETURN_ERROR_PARAM,
            -3 => XCAM_RETURN_ERROR_MEM,
            -4 => XCAM_RETURN_ERROR_FILE,
            -5 => XCAM_RETURN_ERROR_ANALYZER,
            -6 => XCAM_RETURN_ERROR_ISP,
            -7 => XCAM_RETURN_ERROR_SENSOR,
            -8 => XCAM_RETURN_ERROR_THREAD,
            -9 => XCAM_RETURN_ERROR_IOCTL,
            -10 => XCAM_RETURN_ERROR_ORDER,
            -20 => XCAM_RETURN_ERROR_TIMEOUT,
            -21 => XCAM_RETURN_ERROR_OUTOFRANGE,
            _ => XCAM_RETURN_ERROR_UNKNOWN,
        };
        Self(val)
    }
}

impl From<ffi::XCamReturn> for XCamError {
    fn from(val: ffi::XCamReturn) -> Self {
        Self(val)
    }
}

impl std::error::Error for XCamError {}
