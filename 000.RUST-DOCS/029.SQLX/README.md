# SQLX
|摘要|说明|备注|
|-|-|-|
|编译期检查 SQL 的原理|`query!` 是过程宏，不是编译器插件：展开阶段被 rustc 调用时真连数据库拿 `DESCRIBE` 类型表做校验，生成的 `if false { let _: &i32 = &id; }` 假绑定交给 rustc typeck 兜底 | 参考:[000.SQLX为什么能在编译期检查SQL.md](./000.SQLX为什么能在编译期检查SQL.md)|
|-|-|-|
|执行时机|cargo 两阶段构建：先编译 `sqlx-macros` 成 dylib；编译用户 crate 时 rustc 在类型检查**之前**的展开阶段 dlopen 并调用它 | rustc 侧入口：`compiler/rustc_metadata/src/creader.rs:921`|
|-|-|-|
|两道关|sqlx 校验层（`map bind param types`，`?` 与 Rust 类型不匹配 → `span_err`）+ rustc 类型层（假绑定 → E0308）| `cargo expand` 可看到生成代码|
|-|-|-|
|易混淆点|`query_as!(Users, ...)` 是函数式宏不是属性宏；`#[derive(FromRow)]` 才是 derive 宏；sqlx 全用稳定的过程宏 API|-|
|-|-|-|
|当前仓库同类实现|sqlx 不在本仓库，但三种过程宏形式均有本地实例：`compiler/rustc_macros/src/lib.rs:27`（函数式宏，与 `query!` 同路径）、`src/serialize.rs`（derive 宏，对应 `FromRow`）、`src/lib.rs:56`（属性宏）；最小示例见 `tests/ui/proc-macro/auxiliary/` |优先读本地，sqlx 源码走 GitHub 线上引用|
|-|-|-|
|离线/CI|`cargo sqlx prepare` 生成 `.sqlx` 缓存 + `SQLX_OFFLINE=true`；改 SQL 后须重新 prepare|`DATABASE_URL` 缺失时编译直接报错|
|-|-|-|


## 代码示例
##### 编译期参数类型检查
```rust
// users 表: id INT, username VARCHAR(252), version INT
// 宏展开时真连数据库, DESCRIBE users 拿到列类型表, 逐一匹配
async fn fetch(conn: &mut MySqlConnection, id: i32) -> Option<Users> {
    sqlx::query_as!(Users, "SELECT id, username FROM users WHERE id = ?", id)
        .fetch_optional(conn)
        .await
        .unwrap()
}
// 生成代码中埋着假绑定(永不执行但必过 typeck):
// if false {
//     let _: &i32 = &id;   // 列类型锚定: id 必须是 i32, 传 i64 → E0308
// }
```

##### 复现"两道关"
```rust
// 第一道(sqlx 校验层): DESCRIBE 说参数是 INT, 传 &String → 宏内 span_err, 编译中止
// sqlx::query!("SELECT * FROM users WHERE id = ?", name)

// 第二道(rustc typeck): SELECT version 但变量仍是 id → E0308
// sqlx::query!("SELECT version FROM users WHERE id = ?", id)  // version 是 i32, id 也是 i32 → 通过
// let version: String = ...                                   // 用户侧类型不符 → E0308
```

---

## 参考资料
- 微信文章：《Rust sqlx 为什么能在编译期检查 SQL》
- [https://github.com/launchbadge/sqlx](https://github.com/launchbadge/sqlx)
- 过程宏基础：[../025.宏（Macro）/README.md](../025.宏（Macro）/README.md)
