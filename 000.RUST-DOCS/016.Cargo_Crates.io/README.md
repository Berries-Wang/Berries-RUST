# Cargo 和 Crates.io

## 采用发布配置自定义构建
在 `Cargo.toml` 中，可以使用 `[profile]` 部分来定义不同的构建配置。例如：

```toml
[profile.dev]
opt-level = 0  # opt-level 设置控制 Rust 会对代码进行何种程度的优化。这个配置的值从 0 到 3。
debug = true    


# 使用方式: cargo build --release 指定使用 release 配置进行构建
```