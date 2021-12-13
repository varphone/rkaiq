use super::ffi;
use std::borrow::Cow;
use std::ffi::CString;
use std::io;
use std::ptr::NonNull;

pub struct Context {
    pub(crate) internal: NonNull<ffi::rk_aiq_sys_ctx_t>,
}

impl Context {
    pub fn new(sns_ent_name: &str, iq_file_dir: &str) -> Result<Self, io::Error> {
        let sns_ent_name = CString::new(sns_ent_name).unwrap();
        let iq_file_dir = CString::new(iq_file_dir).unwrap();
        let ptr = unsafe {
            ffi::rk_aiq_uapi_sysctl_init(sns_ent_name.as_ptr(), iq_file_dir.as_ptr(), None, None)
        };
        NonNull::new(ptr).map_or_else(
            || Err(io::Error::last_os_error()),
            |v| Ok(Self { internal: v }),
        )
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::rk_aiq_uapi_sysctl_deinit(self.internal.as_ptr());
        }
    }
}

pub struct ContextBuilder<'a> {
    sns_ent_name: Option<Cow<'a, str>>,
    iq_file_dir: Option<Cow<'a, str>>,
}

impl<'a> ContextBuilder<'a> {
    pub fn new() -> Self {
        Self {
            sns_ent_name: None,
            iq_file_dir: None,
        }
    }

    pub fn sns_ent_name<T: Into<Cow<'a, str>>>(mut self, val: T) -> Self {
        self.sns_ent_name = Some(val.into());
        self
    }

    pub fn iq_file_dir<T: Into<Cow<'a, str>>>(mut self, val: T) -> Self {
        self.iq_file_dir = Some(val.into());
        self
    }
}

impl<'a> Default for ContextBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
