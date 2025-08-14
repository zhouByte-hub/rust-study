/**
 * rust可以通过fs::read_dir来遍历目录
 * 
 * fs::read_dir不会递归的遍历目录，只会遍历给定路径下的目录。
 */

#[cfg(test)]
mod self_test {
    use std::{fs, path::PathBuf};


    #[test]
    fn test(){
        let path = PathBuf::from("E:\\project\\rust\\rust-study\\rust_async");

        for item in fs::read_dir(&path).unwrap() {
            let file = item.unwrap();   // 同walkdir一样得到的是DirEntry对象

            println!("==================================================");
            println!("{}", file.file_name().to_string_lossy()); // file_name返回的是OsStr类型
            println!("{:?}", file.file_type());    // fs::FileType类型
            println!("{:?}", file.path());      // 返回的是Path类型
            // 没有判断是否是符号链接和深度的方法，但是可以通过file_type判断是否是符号链接
        }   
        
        // fs::read_link(&path).unwrap() // 读取目录下的符号链接
    }
}