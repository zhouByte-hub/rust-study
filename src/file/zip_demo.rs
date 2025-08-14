/**
 * zip = "4.3.0"
 * 
 * 支持读取和写入 zip 文件的库。
 * 
 *  支持的压缩格式：
 *      1、stored：无压缩
 *      2、deflate
 *      3、deflate64： 仅解压缩
 *      4、bzip2
 *      5、zstd
 *      6、lzma
 *      7、xz 
 *      8、ppmd
 * 
 */

#[cfg(test)]
mod zip_test{
    use std::{fs::File, io::{BufReader, BufWriter, Write}, path::Path};

    #[test]
    fn test_1(){
        let file = File::create(Path::new("src/file/output.zip")).unwrap();
        let buffer = BufWriter::new(&file);

        let mut zip = zip::ZipWriter::new(buffer);
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        // 在压缩文件内创建文件并添加内容
        zip.start_file("name.txt", options).unwrap();
        zip.write("hello zip".as_bytes()).unwrap();

        zip.finish().unwrap();
    }


    #[test]
    fn test_2(){
        let buffer = BufReader::new(File::open("src/file/output.zip").unwrap());
        let mut zip = zip::ZipArchive::new(buffer).unwrap();

        for item in 0..zip.len() {
            let mut file = zip.by_index(item).unwrap();

            let output = Path::new("src/file/output").join(file.name());
            if file.is_dir() {
                std::fs::create_dir_all(&output).unwrap();
            } else if file.is_file() {
                if let Some(parent) = output.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                }
                let mut target = File::create(&output).unwrap();
                std::io::copy(&mut file, &mut target).unwrap();
            }
        }
    }


    #[test]
    fn test_3(){
        let zip_path = Path::new("E:\\project\\rust\\rust-study\\src\\file");

        let file = File::create("E:\\project\\rust\\rust-study\\src\\file\\dir.zip").unwrap();
        let buffer = BufWriter::new(file);
        let mut zip_writer = zip::ZipWriter::new(buffer);

        let options:zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated).unix_permissions(0o755);

        for item in walkdir::WalkDir::new(&zip_path).into_iter().filter_map(|e| e.ok()) {
            let path = item.path();
            /*
                strip_prefix：从一个文件路径中移除指定的前缀路径，返回剩余的相对路径部分

                base = Path::new("/home/user/project")
                path = Path::new("/home/user/project/src/main.rs")
                let relative = path.strip_prefix(base).unwrap();   --> src/main.rs
             */
            let name_in_zip = path.strip_prefix(&zip_path).unwrap();
            if path.is_dir() {
                zip_writer.add_directory(name_in_zip.to_string_lossy(), options).unwrap();
            }else if path.is_file() {
                let mut temp = File::open(&path).unwrap();
                zip_writer.start_file(name_in_zip.to_string_lossy(), options).unwrap();
                std::io::copy(&mut temp, &mut zip_writer).unwrap();
            }
        }
        zip_writer.finish().unwrap();
    }
}