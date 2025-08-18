/**
 * walkdir = "2.5.0"
 *
 * 一个跨平台的 Rust 库，用于高效地递归遍历目录。支持跟随符号链接、控制打开的文件描述符数量以及高效地修剪目录树中的条目。
 */

#[cfg(test)]
mod walkdir_test {
    use walkdir::WalkDir;

    #[test]
    fn test_1() {
        let dir = WalkDir::new("E:\\project\\rust\\rust-study\\rust_async");
        for item in dir {
            let file = item.unwrap(); // 得到的和fs::read_dir一致
            println!("==================================================");
            println!("{}", file.file_name().to_string_lossy()); // file_name返回的是OsStr类型
            println!("{:?}", file.file_type()); // fs::FileType类型
            println!("{:?}", file.path()); // 返回的是Path类型
            println!("{}", file.path_is_symlink());
            println!("{}", file.depth()); // 返回文件所处的深度，从0开始
        }
    }

    #[test]
    fn test_2() {
        // 如果你想要遍历所有条目并忽略可能出现的任何错误，使用 filter_map
        for item in WalkDir::new("E:\\project\\rust\\rust-study\\rust_async")
            .into_iter()
            .filter_map(|e| e.ok())
        {
            println!("{:?}-{}", item.path(), item.depth());
        }
    }

    #[test]
    fn test_3() {
        // 显示符号链接
        for item in WalkDir::new("E:\\project\\rust\\rust-study\\rust_async")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            println!("{:?}-{}", item.path(), item.depth());
        }
    }
}
