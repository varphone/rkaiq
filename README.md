rkaiq
=====

Rockchip RKAIQ bindings for Rust.

Build Examples
--------------

Before you build the examples, you must set `PKG_CONFIG_SYSROOT_DIR` 
to tell `pkg-config` where to find dependencies.

For example: with Full-V RKLASER1 Boards

```sh
export PKG_CONFIG_SYSROOT_DIR=/opt/fullv/2021.02.7-rklaser1/staging
```

Build an run examples:

```sh
cargo r --example minimal
```
