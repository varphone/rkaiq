use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const DEFAULT_RKAIQ_INCLUDE_DIR: &str = "/opt/fullv/2021.02.8-rklaser1/staging/usr/include/rkaiq";
const DEFAULT_TARGET_SYSROOT_DIR: &str = "/opt/fullv/2021.02.8-rklaser1/staging";

#[cfg(feature = "isp-hw-v20")]
const DEFAULT_ISP_HW_VER_DEF: &str = "-DISP_HW_V20=1";
#[cfg(feature = "isp-hw-v21")]
const DEFAULT_ISP_HW_VER_DEF: &str = "-DISP_HW_V21=1";
#[cfg(not(any(feature = "isp-hw-v20", feature = "isp-hw-v21")))]
const DEFAULT_ISP_HW_VER_DEF: &str = "-DISP_HW_V21=1";

fn main() {
    println!("cargo:rerun-if-env-changed=RKAIQ_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=RKAIQ_SYSROOT_DIR");
    println!("cargo:rerun-if-changed=build.rs");

    let rkaiq_include_dir =
        env::var("RKAIQ_INCLUDE_DIR").unwrap_or_else(|_| DEFAULT_RKAIQ_INCLUDE_DIR.into());
    let target_sysroot_dir =
        env::var("TARGET_SYSROOT_DIR").unwrap_or_else(|_| DEFAULT_TARGET_SYSROOT_DIR.into());

    let wrapper_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("wrapper.h");
    let wrapper_path = wrapper_path.to_str().unwrap();
    let mut wrapper = File::create(wrapper_path).unwrap();
    writeln!(wrapper, "#include <rk_aiq.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api_sysctl.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api_imgproc.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api_afec.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_uapi_afec_int.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api_aldch.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_uapi_aldch_int.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api_adpcc.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api_asd.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api_adebayer.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_uapi_adebayer_int.h>").unwrap();
    // writeln!(wrapper, "#include <rk_aiq_tool_api.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_a3dlut.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_abayernr_v2.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_ablc.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_accm.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_acnr_v1.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_acp.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_adebayer.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_adegamma.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_adehaze.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_adpcc.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_adrc.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_ae.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_afec.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_af.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_agamma.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_agic.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_aldch.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_alsc.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_amerge.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_anr.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_asharp_v3.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_atmo.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_awb.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_aynr_v2.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_helper.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_imgproc.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_sysctl.h>").unwrap();
    writeln!(wrapper, "#include <rk_aiq_user_api2_wrapper.h>").unwrap();

    let defines = &[DEFAULT_ISP_HW_VER_DEF];

    let bindings = bindgen::Builder::default()
        .header(wrapper_path)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .anon_fields_prefix("un")
        .derive_debug(true)
        .impl_debug(false)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .impl_partialeq(true)
        .allowlist_function("rk_aiq_.*")
        .clang_args(defines)
        .clang_arg(format!("-I{}/algos/adebayer", rkaiq_include_dir))
        .clang_arg(format!("-I{}/algos/afec", rkaiq_include_dir))
        .clang_arg(format!("-I{}/algos/aldch", rkaiq_include_dir))
        .clang_arg(format!("-I{}/algos", rkaiq_include_dir))
        .clang_arg(format!("-I{}/common", rkaiq_include_dir))
        .clang_arg(format!("-I{}/iq_parser", rkaiq_include_dir))
        .clang_arg(format!("-I{}/iq_parser_v2", rkaiq_include_dir))
        .clang_arg(format!("-I{}/uAPI", rkaiq_include_dir))
        .clang_arg(format!("-I{}/uAPI2", rkaiq_include_dir))
        .clang_arg(format!("-I{}/xcore", rkaiq_include_dir))
        .clang_arg(format!("-I{}", rkaiq_include_dir))
        .clang_arg(format!("--sysroot={}", target_sysroot_dir))
        // .parse_callbacks(Box::new(MyParseCallbacks::default()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=dylib=rkaiq");
}
