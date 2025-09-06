pub(crate) mod declaration_macro_demo;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, ItemFn, LitStr, parse_macro_input};

/**
 * 过程宏允许你编写自定义的宏，这些宏可以在编译时生成或修改代码。过程宏分为三种类型：函数宏、派生宏和属性宏。
 * 文档：https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
 * 
 *  [lib]
 *  proc-macro=true
 *
 *  [dependencies]
 *  syn = { version = "2", features = ["full"] }     // 用来解析语法树(AST)、各种语法构成；
 *  quote = "1.0"   // 解析语法树，生成rust代码，从而实现你想要的新功能；
 *  syn从TokenStream中解析出AST，quote将AST或模板生成TokenStream
 *
 * 过程宏只能定义在lib.rs文件中; 过程宏的输入和输出都是TokenStream类型
 * 不能在定义过程宏的同一个 crate 中使用该过程宏，建议在 tests 目录下创建集成测试来使用此宏
 */

//  函数宏类似于函数调用，使用#[proc_macro]属性定义。
#[proc_macro]
pub fn make_greeting(input: TokenStream) -> TokenStream {
    /* LitStr
        value: String,  // 字符串字面量的值
        span: Span,     // 字符串字面量的位置信息
     */
    let input = parse_macro_input!(input as LitStr);  // LitStr 类型（字符串字面量）
    let name = input.value();

    // 使用 quote 宏生成新的 Rust 代码
    TokenStream::from(quote! {
        fn show(){
            // #name 是模板替换语法，会被前面提取的实际名称值替换
            println!("hello {}", #name);
        }
    })
}

// 派生宏用于自动为类型生成特定的trait实现，使用#[proc_macro_derive]属性定义。
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    /* DeriveInput
        attrs: Vec<Attribute>,  // 应用于目标类型的属性列表
        vis: Visibility,        // 可见性修饰符，如 pub、pub(crate) 等
        ident: Ident,           // 类型的标识符（如结构体名称、枚举名称、函数名称等）
        generics: Generics,     // 类型的泛型参数列表，如 <T>
        data: Data,             // 类型的数据表示，如结构体、枚举等
     */
    let input = parse_macro_input!(input as DeriveInput);   // DeriveInput 用于结构体或者枚举
    let name = &input.ident;
    TokenStream::from(quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("hello macro");
            }
        }
    })
}

// 属性宏用于定义自定义属性，使用#[proc_macro_attribute]属性定义。
#[proc_macro_attribute]
pub fn hello_macro_attr(_: TokenStream, item: TokenStream) -> TokenStream {
    /*  ItemFn
        attrs: Vec<Attribute>,  // 应用于目标函数的属性列表
        vis: Visibility,        // 可见性修饰符，如 pub、pub(crate) 等
        sig: Signature,         // 函数签名，包含函数名称、参数列表、返回类型等
        block: Block,           // 函数体的代码块
     */
    let input = parse_macro_input!(item as ItemFn);     // ItemFn 类型（函数）
    let name = &input.sig.ident;    // 函数名称
    let block = &input.block;   // 函数体的代码块
    let _attrs = &input.attrs;   // 函数的属性列表

    let result = quote! {
        fn #name() {
            println!("hello macro");
            #block
        }
    };
    result.into()
}
