# 宏（Macro）
|摘要|说明|备注|
|-|-|-|
|- println! 就是宏|-|- ```println!("Hello World");```|
|-|-|-|
|- 宏（Macro）|- 宏（Macro）指的是 Rust 中一系列的功能: 使用 macro_rules! 的 [声明宏（declarative macro）](#声明宏)，和三种 [过程宏（procedural macro）](#过程宏)： <br/>1. 自定义 #[derive] 宏，用于在结构体和枚举上通过添加 derive 属性生成代码<br/>2. 类属性宏，定义可用于任意项的自定义属性<br/>3. 类函数宏，看起来像函数，但操作的是作为其参数传递的 token|-|
|-|-|-|
|- 宏和函数的区别|- 从根本上来说，宏是一种为写其他代码而写代码的方式，即所谓的 元编程（metaprogramming）|- 一个函数签名必须声明函数参数的数量和类型。相比之下，宏能够接收可变数量的参数：用一个参数调用 println!("hello") 或用两个参数调用 println!("hello {}", name) <br/> - 宏可以在编译器解析代码前展开，例如，宏可以在一个给定类型上实现 trait。而函数则不行，因为函数是在运行时被调用，而 trait 需要在编译时实现。<br/> - 在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用|
|-|-|-|
|- 使用 macro_rules! 的声明宏用于通用元编程|- 声明宏允许我们编写一些类似 Rust match 表达式的代码|- 参考:[使用 macro_rules! 的声明宏用于通用元编程](#使用-macro_rules-的声明宏用于通用元编程)|
|-|-|-|
|- 宏模式匹配|- 宏定义中有效模式语法和在[第十九章提及的模式语法](../023.模式与模式匹配/000.模式匹配语法.md)是不同的，因为宏模式所匹配的是 Rust 代码结构而不是值|-|
|-|-|-|
|-|-|-|


## 代码示例
### 声明宏
##### 使用 macro_rules! 的声明宏用于通用元编程
声明宏允许我们编写一些类似 Rust match 表达式的代码

宏也<sup>也: match 表达式是一种控制结构，其接收一个表达式，与表达式的结果进行模式匹配，然后根据模式匹配执行相关代码</sup>将一个值和包含相关代码的模式进行比较：此种情况下，该值是传递给宏的 Rust 源代码字面值；模式用于和前面提到的源代码字面值进行比较，一旦匹配成功，每个模式的相关代码会替换传递给宏的代码。所有这一切都发生于编译时。

宏定义中有效模式语法和在[第十九章提及的模式语法](../023.模式与模式匹配/000.模式匹配语法.md)是不同的，因为宏模式所匹配的是 Rust 代码结构而不是值

可以使用 macro_rules! 来定义宏:
```rust
#[macro_export] // #[macro_export] 注解表明只要导入了定义这个宏的 crate，该宏就应该是可用的。如果没有该注解，这个宏不能被引入作用域。
macro_rules! vec { // macro_rules! 和宏名称(vec)开始宏定义
    ( $( $x:expr ),* ) => { // 分支模式 ( $( $x:expr ),* ) ，后跟 => 以及和模式相关的代码块: 如果模式匹配，该相关代码块将被展开
    /**
     * 使用美元符号（$）在宏系统中声明一个变量来包含匹配该模式的 Rust 代码,美元符号明确表明这是一个宏变量而不是普通 Rust 变量。之后是一对括号，其捕获了符合括号内模式的值用以在替代代码中使用。$() 内则是 $x:expr ，其匹配 Rust 的任意表达式，并将该表达式命名为 $x
     * 
     * 在 $() 之后的逗号表示在每个与 $() 内代码匹配的实例之间必须出现一个字面量逗号分隔符。紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式
     * 
     * 当以 vec![1, 2, 3]; 调用宏时，$x 模式与三个表达式 1、2 和 3 对应进行了三次匹配。
     * 
     * 在 $()* 部分，temp_vec.push($x) 会针对模式中每次匹配到 $() 的部分，生成零次或多次，取决于模式匹配到多少次。$x 由每个与之相匹配的表达式所替换。
     * 
     * 当以 vec![1, 2, 3]; 调用该宏时，替换该宏调用所生成的代码:
     * {
     *     let mut temp_vec = Vec::new();
     *     temp_vec.push(1);
     *     temp_vec.push(2);
     *     temp_vec.push(3);
     *     temp_vec
     * }
     */ 
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```


### 过程宏
过程宏接收 Rust 代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出，而非像声明式宏那样匹配对应模式然后以另一部分代码替换当前代码。

定义过程宏的函数接收一个 TokenStream 作为输入并生成 TokenStream 作为输出。TokenStream 是定义于 proc_macro crate 里代表一系列 token 的类型，Rust 默认携带了proc_macro crate。这就是宏的核心：宏所处理的源代码组成了输入 TokenStream，宏生成的代码是输出 TokenStream。函数上还有一个属性；这个属性指明了我们创建的过程宏的类型。在同一 crate 中可以有多种的过程宏。
```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```


##### 如何编写自定义 derive 宏
> 参考代码:[000.RUST-DOCS/025.宏/001.Code-Template/000.自定义derive宏](./001.Code-Template/000.自定义derive宏/)

##### 类属性宏
类属性（Attribute-Like）宏与自定义 derive 宏相似，不同之处在于它们不是为 derive 属性生成代码，而是允许你创建新的属性。

derive 只能用于结构体和枚举；属性还可以用于其它的项，比如函数。

作为一个使用类属性宏的例子，可以创建一个名为 route 的属性用于注解 web 应用程序框架（web application framework）的函数：
```rust
#[route(GET, "/")] // 在 [https://github.com/tokio-rs/tokio]应该有一个 route 属性宏的实现
fn index() { 
    // --snip--
}
```

#[route] 属性将由框架本身定义为一个过程宏。其宏定义的函数签名看起来像这样：
```rust

  # 这里有两个 TokenStream 类型的参数；第一个用于属性内容本身，也就是 GET, "/" 部分。第二个是属性所标记的项：在本例中，是 fn index() {} 和剩下的函数体。
  #[proc_macro_attribute]
  pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
        // --snip--
  }

```

##### 类函数宏
类函数（Function-like）宏的定义看起来像函数调用的宏。类似于 macro_rules!，它们比函数更灵活；

类函数宏获取 TokenStream 参数，其定义使用 Rust 代码操纵 TokenStream，就像另两种过程宏一样。一个类函数宏例子是可以像这样被调用的 sql! 宏：
```rust
   let sql = sql!(SELECT * FROM posts WHERE id=1);
```
这个宏会解析其中的 SQL 语句并检查其是否是句法正确的，这是比 macro_rules! 可以做到的更为复杂的处理。

sql! 宏会被类似于像这样定义：
```rust

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // --snip--
}

```








