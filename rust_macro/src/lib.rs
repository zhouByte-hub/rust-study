pub(crate) mod declaration_macro_demo;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, ItemFn, LitStr, parse_macro_input};

/**
 * 过程宏允许你编写自定义的宏，这些宏可以在编译时生成或修改代码。过程宏分为三种类型：函数宏、派生宏和属性宏。
 *  [lib]
 *  proc-macro=true
 *
 *  [dependencies]
 *  syn = { version = "2", features = ["full"] }     // 用来解析语法树(AST)、各种语法构成；
 *  quote = "1.0"   // 解析语法树，生成rust代码，从而实现你想要的新功能；
 *
 * 过程宏只能定义在lib.rs文件中; 过程宏的输入和输出都是TokenStream类型
 * 不能在定义过程宏的同一个 crate 中使用该过程宏，建议在 tests 目录下创建集成测试来使用此宏
 */

//  函数宏类似于函数调用，使用#[proc_macro]属性定义。
#[proc_macro]
pub fn make_greeting(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
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
    let input = parse_macro_input!(input as DeriveInput);
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
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;

    let result = quote! {
        fn #name() {
            println!("hello macro");
            #block
        }
    };
    result.into()
}
