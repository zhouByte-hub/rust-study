fn main() {
    // 使用 UI Design 库进行编译
    // let config = slint_build::CompilerConfiguration::new().with_library_paths(
    //     std::collections::HashMap::from([(
    //         "surrealism".to_string(),
    //         std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
    //             .join("ui/surrealism-ui/index.slint"),
    //     )]),
    // );
    // slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
