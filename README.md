rkaiq
=====

Rockchip RKAIQ bindings for Rust.

Build Examples
--------------

Before you build the examples, you must set the env:

- `PKG_CONFIG_SYSROOT_DIR`
- `RKAIQ_INCLUDE_DIR`
- `TARGET_SYSROOT_DIR`

to tell `pkg-config` and `bindgen` where to find dependencies.

For example: with Full-V RKLASER1 Boards

```sh
export PKG_CONFIG_SYSROOT_DIR=/opt/fullv/2021.02.8-rklaser1/staging
export RKAIQ_INCLUDE_DIR=/opt/fullv/2021.02.8-rklaser1/staging/usr/include/rkaiq
export TARGET_SYSROOT_DIR=/opt/fullv/2021.02.8-rklaser1/staging
```

Build an run examples:

```sh
cargo r --example minimal
```

Features
--------

- `fullv` - Enable Full-V patches.
- `isp_hw_v20` - Build for ISP_HW V20 (RV1126, RV1109)
- `isp_hw_v21` - Build for ISP_HW V21 (RK356X)
- `isp_hw_v30` - Build for ISP_HW V30 (RK3588)
- `isp_hw_v31` - Build for ISP_HW V31 (???)
- `v1_0` - Build with RKAIQ 1.0
- `v2_0` - Build with RKAIQ 2.0
- `v3_0` - Build with RKAIQ 3.0
