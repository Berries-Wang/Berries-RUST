# Rust


## 参考资料
+ https://mp.weixin.qq.com/s/SgdVqi0sl7g5ziamBazJOA — Rust 1.97 修复了 pin!() 的一个隐蔽未定义行为：Deref 强制转换如何绕过 Pin 的保证
+ https://mp.weixin.qq.com/s/esaBYMVx-bVCHBbDysJX8w — Rust 性能调优实战：hotpath-rs 从埋点到 CI 防劣化完全指南
+ https://mp.weixin.qq.com/s/J2CPrHAtpgCDXjsZlm37Dg — Polonius 正在走近稳定：Rust 借用检查器的下一站
+ https://mp.weixin.qq.com/s/-4DoLQSrBYO9Cc_8-3yCPA — Rust `use` 关键字深度剖析：构建清晰模块路径的艺术与最佳实践
+ https://mp.weixin.qq.com/s/3AKY-YXURgzAmDoT41Tu_A — Rust #[path] 属性深度解析：打破模块约定的利与弊，及最佳实践决策框架
+ https://mp.weixin.qq.com/s/QzMfvBxFy1YQJZK4h80m6A — #[diagnostic::on_unknown] 进入模块：Rust 自定义错误消息的精细化控制
+ https://mp.weixin.qq.com/s/nSlc7BBPkfsBbSC1W_Jkzg — Rust 的 Range 类型终于可以 Copy 了：core::range 模块的价值
+ https://mp.weixin.qq.com/s/O4JtEtyOA1R7iChbINAMbw — 从 matches!() 到 assert_matches!()：Rust 断言机制的一次补齐
+ https://mp.weixin.qq.com/s/RgYqbZD2piGlcqKKAtjpAA — Rust 网络编程：从 TCP 到 QUIC
+ https://mp.weixin.qq.com/s/sUeXNACBMYRtdPiGtzS1iA — build.rs 是什么？看完这篇你就懂了
+ https://mp.weixin.qq.com/s/gNQBlPiz5GgDL7c2JUmCGg — Rust CRC32 三强对决：`crc32c` vs `crc` vs `crc-fast`——性能差距竟达 200 倍，如何精准选型？
+ https://mp.weixin.qq.com/s/T9AfubIUOjwNRP7PRPHNHg — Rust 并发同步原语 `AtomicBool` 与 `LazyLock`：从理论到实战的高性能指南
+ https://mp.weixin.qq.com/s/CbQF9Q23QietTvGH5F4P2g — Rust 的 `only` 约束：`?Sized` 的继任者来了
+ https://mp.weixin.qq.com/s/5SmdF_e-i6_KETW28B2VhA — 当 panic 和 wrapping 都不够：Rust 的可恢复整数溢出机制
+ https://mp.weixin.qq.com/s/FfxUjg7Cnqab6dyrV0Lt1g — Rust 分布式系统请求标识命名最佳实践：从理论到高性能分层指南
+ https://mp.weixin.qq.com/s/V4Uvix3dOpxcKHsFFsCVTw — 告别闭包前的克隆样板：Rust move 显式捕获语法进入 RFC
+ https://mp.weixin.qq.com/s/MtpoADIqhT-XeUs5JNaf4Q — Rust 网络编程进阶：协议无关请求管线 —— 让 HTTP/3 与 WebTransport 共享网关治理
+ https://mp.weixin.qq.com/s/QuPVkLjBBpkZAjbC2U04nA — Rust CRC32 选型终极指南：`crc` vs `crc-fast` 深度对比，性能相差 200 倍？
+ https://mp.weixin.qq.com/s/MAv8r6Trjl_NmzWE1VqvKA — Rust 常量泛型的 2026 关键突破：mGCA 接近稳定，类型级计算加速落地
+ https://mp.weixin.qq.com/s/9ioo2KRuZ0m5Wk65lbNLqA — Rust 内存可观测性最佳实践指南：从分配器选型到火焰图的全链路高性能方案
+ https://mp.weixin.qq.com/s/wYuHhnHV97MSAJxAAqey4g — 深入 Rust 元编程：syn 解析库从入门到精通
+ https://mp.weixin.qq.com/s/BJhOkuH13r4LbcFCKosqtQ — Rust Pin Drop 语法糖来了：`fn drop(&amp;pin mut self)` 即将合入
+ https://mp.weixin.qq.com/s/UYdULvLAEJw0lgyl0kC2lw — RustFS 性能重塑：从 jemalloc 到 mimalloc 的平滑迁移与 pprof 全栈火焰图方案
+ https://mp.weixin.qq.com/s/Jo0LNzm_Yw96CGrnqwVD3A — Rust 的 comptime 落地了：编译期函数执行与类型反射进入新阶段
+ https://mp.weixin.qq.com/s/ZhW9ynU8HF_tSd6hAvilrw — Cargo 的诊断系统正在从文本走向结构化
+ https://mp.weixin.qq.com/s/aUmeIB2ha5uBDk6iphwibQ — 自定义指针的「借」与「再借」：Rust Reborrow Trait 如何填补引用语义的缺口
+ https://mp.weixin.qq.com/s/snEoFxw-ty3DL8-FMj_HOw — 当编译器标志导致未定义行为：Rust 的 Target Modifier 机制如何堵上 ABI 安全缺口
+ https://mp.weixin.qq.com/s/5LJGQGtdU5RU7MK3rvUtCQ — 从标准库学 unsafe：Rust 的指针来源检查正在统一收紧
+ https://mp.weixin.qq.com/s/xnLTE26I595kB3uLAr1x8g — Rust 类型系统的「下一代」正在走向稳定：新 trait solver 和 Polonius 进入 CI 测试
+ https://mp.weixin.qq.com/s/L5nZhl7rq3gr096HkneBBQ — Rust 标准库的 GPU 计算之路：std::offload 如何让 GPU 编程成为语言的一等公民
+ https://mp.weixin.qq.com/s/AN-2WtgTv2qt2GNYDw52OQ — Rust 标准库正引入 Share trait：为引用计数类型建立「克隆即别名」语义层
+ https://mp.weixin.qq.com/s/l4jh8ePUsXDX5uhaQ7mk_g — 整数类型转换的 API 革命：Rust 正在为每个整数加上五种 cast 方法
+ https://mp.weixin.qq.com/s/6vWGw72PqBxyYI8v3D5rjQ — Rust 标准库正在引入复数类型：一个 lang item 背后的 C 语言兼容工程
+ https://mp.weixin.qq.com/s/bKL7qTPmxmMtQ7PS_bvAVg — LiteParse：为AI Agent而生的本地文档解析引擎——从技术原理到生产实践
+ https://mp.weixin.qq.com/s/NUE0EknVgXneaOZycQNpHw — Box 和 Vec 迎来 NonNull 通道：Rust 标准库的低层安全编程拼图
+ https://mp.weixin.qq.com/s/S68bdiQ6PW-r317VVlHiNg — Rust 的 reuse 关键字：函数委托如何终结 newtype 模板代码
+ https://mp.weixin.qq.com/s/_AomQ3ZAIBND1F_f6alsNg — Rust 的稳定性承诺正在进化：--allow-unstable-flags 如何终结 RUSTC_BOOTSTRAP 的暴力时代
+ https://mp.weixin.qq.com/s/c85dBe_5CtHNpZ-eMo2YHA — Rust 零拷贝技术从入门到精通——量化性能收益与完整实战指南
+ https://mp.weixin.qq.com/s/hB2TtEldZkESxj-yjr4O_A — Rust 编译器可以为循环&quot;手动挡&quot;了：#[unroll] 进入 Nightly
+ https://mp.weixin.qq.com/s/I1nwg5YZg93ld4grqSmI9w — 告别 println! 调试地狱，macOS Rust 开发者都该学会 rust-lldb
+ https://mp.weixin.qq.com/s/vA4b43htJCk1zAr0tIo-mw — 条件编译不用再装 crate 了：Rust 1.95 把 cfg_if 吸进了标准库
+ https://mp.weixin.qq.com/s/dNtZrw68aOLeWfLRHk5mUQ — Rust 的 const generics 走向&quot;完全体&quot;：当泛型参数不再止于整数
+ https://mp.weixin.qq.com/s/HJiuUv7zMoTGCbL2UdabXA — 零拷贝的内核对决：在 RustFS 中的 io_uring 网络传输 vs 普通上传/下载深度对比
+ https://mp.weixin.qq.com/s/3cFCrmR25k-rD8mS3-IYeA — Cargo target/ 目录自动垃圾回收来了：cargo clean gc 背后的缓存管理
+ https://mp.weixin.qq.com/s/Kv94wF_19aq_eZhGkVfBOw — Cargo 配置 include 稳定后：AI 改 Rust 项目，先把工具链策略分层
+ https://mp.weixin.qq.com/s/kqFfzVNPTbwxARUZdj98YQ — cfg_select! 进入标准库：当&quot;少一个依赖&quot;背后是 Rust 条件编译的工具链升级
+ https://mp.weixin.qq.com/s/3yHl7ZJ-ilOncfGnZ9sYXw — Rust 1.95：条件编译的标准化时刻
+ https://mp.weixin.qq.com/s/sERR4hRJ_-CYSt5i_PBGxQ — cfg_select! 高级实战：构建零开销的跨平台抽象层
+ https://mp.weixin.qq.com/s/K4e_bRthAQ_Ku7OfOx_WIw — LLVM 正在编译你用不到的代码：Rust 的死函数消除优化即将到来
+ https://mp.weixin.qq.com/s/rSeS4PWjfLbL3dpY224TCg — Rust 标准库的 Copy 化进程：从 Range 到核心类型的零成本可复制
+ https://mp.weixin.qq.com/s/bdVquoGoRb3tHZXwcRYUtg — 改个常量就要重编六秒？inline_tweak 给 Rust 带来的运行时&quot;调参&quot;能力
+ https://mp.weixin.qq.com/s/2kL6dr-hNxh830TbpYkvIQ — Rust 1.96.0 版本发布：宏系统增强、Wasm 链接优化、安全修复及 Cargo 新功能
+ https://mp.weixin.qq.com/s/_EXG_EK-sJv5G0vCndEnDg — Rust 网络编程进阶：打通 gRPC over HTTP/3 —— 在网关层实现 HTTP/3 入站与 HTTP/2 上游桥接
+ https://mp.weixin.qq.com/s/Vq1fCD14GTtgUruOaT9tbw — Rust 惰性初始化家族图谱：OnceCell、LazyLock、From 与各取所需的工程选择
+ https://mp.weixin.qq.com/s/7Z9Ygd5ku6-XGDVuog6X2Q — Rust Cow 的性能代价：写时克隆不是零成本的
+ https://mp.weixin.qq.com/s/qKIhnokGjT7_5e3Kujnl0w — Cargo 配置系统正在从&quot;扁平 ini&quot;走向&quot;条件表达式&quot;
+ https://mp.weixin.qq.com/s/QYoh66tV61CoaFFInCxpEQ — Rust 错误上下文丢失问题：从 anyhow 到作用域化错误的工程演进
+ https://mp.weixin.qq.com/s/BfoiO0zptR0ml3sarJjIYA — Rust 的跨平台 SIMD 编程：std::simd 的生产力之选
+ https://mp.weixin.qq.com/s/luNFIZAb4Wip4HrQF5DHcw — Cargo 的网络层正在经历一次无声的工程迁移
+ https://mp.weixin.qq.com/s/B6pkEjAwCgrag9ntu1e6UA — Rust 的 Niche 优化：Option 的零开销抽象是如何炼成的
+ https://mp.weixin.qq.com/s/E1NnhEWsKm_uj3Z1GC8SDw — Rust 1.96 对无值静态变量的零容忍：uninhabited_static 升级意味着什么
+ https://mp.weixin.qq.com/s/KZELoPQv2QhSZGHB-XrqAg — Rust 1.96 补上了 proc macro 最后一块 hygiene 拼图：$crate
+ https://mp.weixin.qq.com/s/ggRG_biQIv7lTNPqt-yecg — Rust 1.96 修复了 Pin 的一个隐式类型漏洞：为什么 Pin 需要 Deref
+ https://mp.weixin.qq.com/s/Yiq6KX2EDjIdsqBPNM4K7g — trait 方法里 impl Trait 的可见性守卫：Rust 1.96 的隐私检查
+ https://mp.weixin.qq.com/s/Pm3hIzDxH6NB6h5XWCtE5A — Rust 正在实验「开放枚举」：用 _ = .. 语法解决 C enum 互操作难题
+ https://mp.weixin.qq.com/s/sgVDI_-ZxvHW9GbqZ4E16Q — 同时修改两个数组元素，Rust 终于给了安全答案
+ https://mp.weixin.qq.com/s/o26H7x4XZVf4l5toWEbWJw — 从 impl const 到 const impl：Rust const traits 接近稳定前的最后一次语法调整
+ https://mp.weixin.qq.com/s/onloSbXUW1SjvNxSN87lbQ — 当 assert!(matches!(...)) 只告诉你&quot;失败了&quot;——Rust 1.96 的 assert_matches! 终于能...
+ https://mp.weixin.qq.com/s/1o13buyPXRKxPQU_wHNuCw — Rust 条件编译的宏困局：一个 expr 如何打破多年的限制
+ https://mp.weixin.qq.com/s/5CnJbxJQIaH8n4TA4IeJOg — 当 repr(C) 不再只是 C：Rust 为结构体布局拆分出一个新 repr
+ https://mp.weixin.qq.com/s/SG-bsMs9eRkEFVjoUK_wQQ — Rust Allocator trait 十年稳定路：自定义分配器终于可以在稳定版上用了
+ https://mp.weixin.qq.com/s/nYrymm34yiO1DHwghKn_MQ — Rust 1.96 稳定新范围类型：.. 和 ..=5 终于有了专属身份
+ https://mp.weixin.qq.com/s/4MkkXs59zetZTqFVKf1aWg — Rust 图像处理优化战记：51ms 到 8.6ms，fast_blur 的 5 倍加速之路
+ https://mp.weixin.qq.com/s/aWi5hmIG1EAvU1MHbihNwQ — try_as_dyn：Rust 运行时类型查询正在走向成熟
+ https://mp.weixin.qq.com/s/shA8JJBLJ2Zvk2gixMVvIA — Rust 对齐 ARM 向量扩展：SVE/SME 原生支持进入 Nightly
+ https://mp.weixin.qq.com/s/b2gR71UjUOLgX9gWk8Co8g — Rust 编译期反射来了：std::reflect 的 MVP 意味着什么
+ https://mp.weixin.qq.com/s/APn4CEmb4SBs_lJUITY7dw — Rust 正在接管 AI Agent 的基础设施层
+ https://mp.weixin.qq.com/s/YDr_s9YEKYHNCrTijMf_rg — Rust C++ 互操作加速：Foundation 专职到岗、splat 实验启动，跨语言边界正在被系统性地重新绘制
+ https://mp.weixin.qq.com/s/lEFS5POnvyRZeYWNiJL3aQ — `move(expr)` 进入 Rust 编译器：按值捕获的精确控制时代
+ https://mp.weixin.qq.com/s/yL309pGeI9DujA0sK7m2Vw — assert_matches! 正式可用：Rust 测试中的模式匹配终于有了原生断言
+ https://mp.weixin.qq.com/s/o9lFc6G9tI4CHDmBvHuV9w — Rust 类型系统的新维度：Field Projections 让结构体字段成为一等类型公民
+ https://mp.weixin.qq.com/s/ZTohEMRm8Eyx9RjKbngvng — Cargo 工作区依赖终于支持本地覆盖 default-features 了
+ https://mp.weixin.qq.com/s/PNPDvoJZ_WH93mC8CN10ew — 5 倍加速：一次 Rust 图像模糊函数的极致优化
+ https://mp.weixin.qq.com/s/yZhyY2VRcIxfAVodDOySDg — Rust 的 MIR 移动消除：编译器开始消灭不必要的赋值
+ https://mp.weixin.qq.com/s/K_HGbdPWWLBZ2nVDvc2p3A — Vec 到数组的快速通道：Rust 标准库 into_array 填补长期空缺
+ https://mp.weixin.qq.com/s/xM4doISd3j6bbWyRFROzlA — Rust 的 trait 对象间安全转型：Bounded Trait Casting RFC 进入讨论
+ https://mp.weixin.qq.com/s/Jp9J1tZv5RQq3cc2Zsjj0g — cfg_select! 进了标准库：条件编译这件事，Rust 不想再让社区自己拼了
+ https://mp.weixin.qq.com/s/tSxny6g-dBXe3W8_VEbO_w — Cargo 的新安全防线：text_direction_codepoint lint 如何阻断视觉欺骗攻击
+ https://mp.weixin.qq.com/s/w7Sa0xsZ-weTI3H-2p5zpA — 给回调参数起个名字：Fn(msg: String) 语法正式进入 Rust
+ https://mp.weixin.qq.com/s/FjOeCPg2QDJXd_j0hSWmwQ — Rust 编译期反射 MVP 落地：零成本类型自省的起点
+ https://mp.weixin.qq.com/s/c_SZ1uRbdhxxb2HX69mqmg — Rust 字段投影（Field Projections）初探：当编译器开始理解字段本身
+ https://mp.weixin.qq.com/s/vxIL2oP7K_R-lBJXGIN8fg — Rust 的类型系统开始理解结构体字段：field_of! 宏与 Field Projections 解读
+ https://mp.weixin.qq.com/s/_KNoElhB3H9x47LcybWKlA — Rust 的 build-std 终于要走向稳定了：从零编译标准库的最后冲刺
+ https://mp.weixin.qq.com/s/Nw59zbZExv_JvPnrnjREHw — match 分支的 if let 守卫：Rust 模式匹配的最后一块拼图
+ https://mp.weixin.qq.com/s/LrvG1Xg7fkFhUwJdAeAn6A — Rust 运行时检测能力升级：MSan 和 TSan 走向稳定
+ https://mp.weixin.qq.com/s/RVOzlq5kgU3hb2dW4vXHXA — Cargo 构建分析里程碑：cargo report rebuild 与结构化日志实战
+ https://mp.weixin.qq.com/s/IjOjKKcsBDnZ_fFLbPcyEQ — Rust 编译器并行前端：编译速度的新突破
+ https://mp.weixin.qq.com/s/xjlEb3zDt9Qd3DvkpzsWPw — field_of! 进入 nightly：Rust 结构体字段的&quot;类型级身份&quot;来了
+ https://mp.weixin.qq.com/s/uS1pNMOELT8jBZMhusTPtQ — Cargo Script 即将稳定：Rust 终于可以像脚本一样运行了
+ https://mp.weixin.qq.com/s/M7BCFRi4vAc-5i1h91M-ow — Reborrow 特质：把重借用能力赋予自定义引用类型
+ https://mp.weixin.qq.com/s/PyNg9trxk147ecWQ_KoigQ — 改一个私有函数，不必重编译全世界：Rust Relink Don&#39;t Rebuild 深入解读
+ https://mp.weixin.qq.com/s/G2qcngOy0tLDwhIVsxAm8Q — Rust 编译器并行前端：从单线程到多核加速的跨越
+ https://mp.weixin.qq.com/s/I6sFZxsaCNdwBDffNjEE9Q — Rust 新一代 trait solver 进入 CI 测试：类型系统的核心正在被替换
+ https://mp.weixin.qq.com/s/WDJfXyqvOfIwOrAyECqp8w — cfg_select!：Rust 1.95 把编译时条件分支带进了标准库
+ https://mp.weixin.qq.com/s/OJB7SMe87bLa22-R2rBUng — mistral.rs v0.8.0：Rust 推理框架从「跑模型」到「让模型干活」
+ https://mp.weixin.qq.com/s/V4KNZ5eNgLnr2Rk5-F2rQA — Rust I/O 拆入 core：no_std 异步碎片化收场，端侧推理等到标准 I/O
+ https://mp.weixin.qq.com/s/kHmCQGraUCpcx0To5oE9jA — Rust 正在实验&quot;字段级借用&quot;：View Types 初探
+ https://mp.weixin.qq.com/s/5EvfsLbgv3nAKXkTxpEuaw — Share trait 与 move 表达式：Rust 引用计数编程体验即将升级
+ https://mp.weixin.qq.com/s/wfptz2J_-7wbruXSMqnl1g — cfg_select! 来了：Rust 条件编译终于不再依赖第三方 crate
+ https://mp.weixin.qq.com/s/jsMOhsBoakLXeLOJ89sRyg — 从 boxed future 到 async closure：Rust 异步编程的一次静默进化
+ https://mp.weixin.qq.com/s/d3r5SafON_JCSjroEjCRxg — Rust 也能写脚本了：cargo script 即将稳定，一个 .rs 文件搞定依赖与运行
+ https://mp.weixin.qq.com/s/ovgWfg7N76mp1Gme4O6ajw — Rust C++ 互操作从调研走向落地：Foundation 的 Interop Initiative 新进展
+ https://mp.weixin.qq.com/s/fgaL0bXXWpeSqw3MPpUklw — cfg_select! 稳定了：cfg-if 近十亿次下载的使命，编译器自己接
+ https://mp.weixin.qq.com/s/JvBeRFsR3PXHGMlw7WX0cQ — Rust try 块进入新阶段：当不同错误类型可以在同一个块内共存
+ https://mp.weixin.qq.com/s/W7Mt4YjKOVi__tZ7R0RrIw — Rust 的 newtype 模式终于要成为一等公民了
+ https://mp.weixin.qq.com/s/-LWYPUX9UOdRWP3oU5rd3A — 一行 &amp; 和 clone()，决定你是不是真的懂 Rust
+ https://mp.weixin.qq.com/s/HQ69vTB10YDEHZgETD7vrQ — Rust 日志系统极限优化：双轮转引擎 + 零阻塞清理的生产级实战
