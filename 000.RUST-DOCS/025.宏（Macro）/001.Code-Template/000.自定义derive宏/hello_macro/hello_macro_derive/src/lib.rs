use proc_macro::TokenStream;
use quote::quote;

/*
 * 代码分成了 hello_macro_derive 和 impl_hello_macro 两个函数，前者负责解析 TokenStream，
 * 后者负责转换语法树：这使得编写过程宏更加方便
 * 
 * proc_macro crate 是编译器提供用来读取和操作我们 Rust 代码的 API
 * 
 * 当用户在一个类型上指定 #[derive(HelloMacro)] 时，hello_macro_derive 函数将会被调用。
 * 
 * 使用 proc_macro_derive 注解该函数并指定名称 HelloMacro，该名称与我们的 trait 名称相匹配；
 * 
 * 
 */
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {

    /*
     * 该函数首先将来自 TokenStream 的 input 转换为一个我们可以解释和操作的数据结构。
     * 这正是 syn 派上用场的地方。syn 中的 parse 函数获取一个 TokenStream 并返回一个表示解析出的 Rust 代码的 DeriveInput 结构体。
     * 
     * 
     * 当调用 syn::parse 函数失败时，我们用 unwrap 来使 hello_macro_derive 函数 panic。
     * 在错误时 panic 对过程宏来说是必须的，因为 proc_macro_derive 函数必须返回 TokenStream 而不是 Result，
     * 以此来符合过程宏的 API。这里选择用 unwrap 来简化了这个例子；
     * 在生产代码中，则应该通过 panic! 或 expect 来提供关于发生何种错误的更加明确的错误信息。
     */
    // 将 Rust 代码构建成我们可以操作的语法树。
    let ast = syn::parse(input).unwrap();

    // 生成 trait 的实现
    impl_hello_macro(&ast)
}
/**
 * 定义 impl_hello_macro 函数，其用于构建所要包含在内的 Rust 新代码
 * 
 * @return 其输出也是 TokenStream。所返回的 TokenStream 会被加到我们的 crate 用户所写的代码中，
 *         因此，当用户编译他们的 crate 时，他们会通过修改后的 TokenStream 获取到我们所提供的额外功能
 * 
 * 
 */
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    /*
     * quote! 宏能让我们编写希望返回的 Rust 代码。
     * quote! 宏执行的直接结果并不是编译器所期望的所以需要转换为 TokenStream。
     * 为此需要调用 into 方法，它会消费这个中间表示（intermediate representation，IR）并返回所需的 TokenStream 类型值
     * 
     * quote! : [000.RUST-DOCS/025.宏/000.Crate quote.md]
     * 
     * stringify! 为 Rust 内置宏。其接收一个 Rust 表达式，如 1 + 2 ，然后在编译时将表达式转换为一个字符串常量，如 "1 + 2" 。
     */
    let generated = quote! {
        // 当写 #name 时，quote! 不会把它当成字面量的 #name，而是去当前作用域找一个叫 name 的变量，把那个变量的内容拼贴进去
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}