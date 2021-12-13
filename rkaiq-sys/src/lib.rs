//! RKAIQ ISP 应用接口安全绑定。
//!
//! 本项目当前基于 RKAIQ V1.0 ISP 应用接口接口实现。
//!
#![allow(deref_nullptr)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unaligned_references)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
