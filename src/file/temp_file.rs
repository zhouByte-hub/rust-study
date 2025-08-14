/**
 * tempfile = "3.20.0"
 * 一个安全的、跨平台的 Rust 临时文件库。除了创建临时文件外，该库还允许用户安全地打开对同一临时文件的多个独立引用（适用于生产者/消费者模式，且安全实现起来出乎意料地困难）。
 */

#[cfg(test)]
mod temp_test{
    use std::io::{Read, Write};


    #[test]
    fn temp_file_test() {
        // windows系统中会将临时文件创建在temp目录下：C:\\Users\\Time_Travel\\AppData\\Local\\Temp\\.tmpu6yjF
        let mut temp_file = tempfile::tempfile().unwrap();
        println!("temp_file Path = {:?}", temp_file);

        let content = "hello temp_file";
        temp_file.write(content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let mut read_content: [u8; 1024] = [0; 1024];
        let length = temp_file.read(&mut read_content).unwrap();
        temp_file.flush().unwrap();

        println!("{:?}", String::from_utf8(read_content[0..length].to_vec()));
    }


    #[test]
    fn temp_dir_test(){
        let temp_dir = tempfile::tempdir().unwrap();
        println!("{:?}", temp_dir.path());
        // temp_dir.close().unwrap();
    }


    #[test]
    fn temp_file_in_temp_dir(){
        let dir = tempfile::tempdir().unwrap();
        let mut file = tempfile::tempfile_in(&dir).unwrap();

        println!("{:?}", file);

        file.write("hello world".as_bytes()).unwrap();
        file.flush().unwrap();
    }


    #[test]
    fn read_test(){
        let content = std::fs::read_to_string("C:\\Users\\Time_Travel\\AppData\\Local\\Temp\\.tmpABNFP2\\.tmpDktaZJ").unwrap();
        println!("{}", content);
    }
}