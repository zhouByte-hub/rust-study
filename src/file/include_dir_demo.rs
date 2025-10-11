/**
 * include_dir = {version = "0.7.4", features=["glob"]}
 *
 * 将目录内容嵌入到你的二进制文件中
 * 对 include_str!() 和 include_bytes!() 宏的演进，用于将整个目录树嵌入到你的二进制文件中。
 *
 * 你传递给宏一个文件路径，并将返回的值赋给某个 static 变量。
 */

#[cfg(test)]
mod include_test {
    use std::path::Path;

    use include_dir::{Dir, include_dir};

    // 在编译时会判断文件是否存在，因为需要被编译到二进制中
    #[cfg(target_os = "macos")]
    static FILE_DIR: Dir = include_dir!("src/file");

    #[cfg(target_os = "windows")]
    static FILE_DIR: Dir = include_dir!("E:\\project\\rust\\rust-study\\src\\file");

    #[test]
    fn test() {
        let rust_self = FILE_DIR.get_file(Path::new("rust_self.rs")).unwrap();
        let content = rust_self.contents_utf8().unwrap();

        println!("{}", content);
    }

    #[cfg(target_os = "macos")]
    static SERIALIZE_DIR: Dir =
        include_dir!("src/serialize");

    #[cfg(target_os = "windows")]
    static SERIALIZE_DIR: Dir = include_dir!("E:\\project\\rust\\rust-study\\src\\serialize");
    #[test]
    fn pattern_test() {
        for item in SERIALIZE_DIR.find("**/*.ini").unwrap() {
            let path = item.path();
            println!("{:?}", path);
        }
    }
}
