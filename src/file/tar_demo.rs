/**
 * tar = "0.4.44"
 * 
 * 一个用于读取和写入 TAR 文件的 Rust 实现。
 * 该库目前不处理压缩，但它对所有 I/O 读取器和写入器进行了抽象。此外，还采取了极大的措施来确保整个内容永远不会一次性全部驻留在内存中。
 */

#[cfg(test)]
mod tar_test{
    use std::fs::File;

    use tar::{Archive, Builder};


    #[test]
    fn write_test(){
        let file = File::create("src/file/tar_output.tar").unwrap();
        let mut build = Builder::new(file);

        for item in std::fs::read_dir("src/file").unwrap() {
            let sub_file = item.unwrap();
            build.append_path(sub_file.path()).unwrap();    // 根据路径来添加文件
            // build.append_file(path, file)   // 添加路径下的指定文件
            // build.append_dir(path, src_path)
        }
    }


    #[test]
    fn read_test(){
        let file = File::open("src/file/tar_output.tar").unwrap();
        let mut archive =  Archive::new(file);
        for item in archive.entries().unwrap(){
            let file = item.unwrap();
            println!("{:?}-{:?}", file.path().unwrap(), file.header().size());
        }
    }
}