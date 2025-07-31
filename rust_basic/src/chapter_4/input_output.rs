use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Read, Seek, Write},
    path::PathBuf,
};

/** Rust中的输入输出流
 *  1、实现Read特性的类型具有以字节为导向的输入方法。
 *      1.1、std::fs::File::open(filename)：用于打开文件
 *      1.2、std::net::TcpStream：用于从网络接收数据。
 *      1.3、std::io::stdin()：用于从进程的标准输入流读取数据。
 *      1.4、std::io::Cursor<&[u8]> 值：从内存的字节数组中 “读取” 数据。
 *
 *  2、实现Write特性的类型支持以字节为导向的和UTF-8文本输出。
 *      2.1、std::fs::File::create(filename)：用于打开文件。
 *      2.2、std::net::TcpStream：用于通过网络发送数据。
 *      2.3、std::io::stdout() 和 std::io::stderr()：用于将数据写入终端。
 *      2.4、std::io::Cursor<&mut [u8]>：允许将任何可修改字节切片作为文件写入
 *      2.5、Vec<u8>：也是一个写入器，它的 write 方法可以为向量追加元素。
 */

// 标准输入
pub fn stdio_test() {
    let mut content = String::new();
    let length = std::io::stdin().read_line(&mut content).unwrap(); // 程序会等待，直到用户输入

    println!("键盘输入的内容为：{}, 长度为：{}", content, length); // 长度一般会加2，因为最后会存在\n\r
}

// 标准输出
pub fn stdout_test() {
    let content = String::from("content");
    let length = std::io::stdout().write(content.as_bytes()).unwrap();
    println!("length = {}", length);
}

// 文件输入
pub fn file_in_test() {
    /*
     * std::fs::read(PathBuf::from("src/chapter_4/inputTestFile.txt")) // 直接读取文件，但是返回的是字符
     * std::fs 和 std::fs::File很多方法都相同，但是File提供的更加丰富
     */
    let mut file = std::fs::File::open(PathBuf::from("src/chapter_4/inputTestFile.txt")).unwrap();
    let mut content = String::new();
    let length = file.read_to_string(&mut content).unwrap();

    println!("{}-{}", content, length);
}

// 文件输出
pub fn file_out_test() {
    let content = "javaScript \nVue";

    // 如果使用这种方法打开一个文件然后进行写入，会报错： PermissionDenied, message: "拒绝访问。"
    // let mut file = std::fs::File::open(PathBuf::from("src/chapter_4/inputTestFile.txt")).unwrap();
    // let _ = file.write(content.as_bytes()).unwrap();

    // 可以使用options来给对象赋权
    let mut file = std::fs::File::options()
        .read(true)
        .write(true)
        .append(true)
        .open(PathBuf::from("src/chapter_4/inputTestFile.txt"))
        .unwrap();
    let _length = file.write(content.as_bytes()).unwrap();

    // 也可以这么写
    // OpenOptions::new().read(true).write(true).append(true)
}

/**
 * 在 Rust 中，BufReader 和 BufWriter (注意是 BufWriter，不是 BufWrite) 是 std::io 模块提供的用于缓冲输入和输出的包装器。
 * 它们的主要目的是通过减少对底层 I/O 资源（如文件、网络套接字、标准输入/输出）的系统调用次数来显著提高 I/O 性能。
 *
 * 无缓冲 I/O: 每次调用 read 或 write 方法时，都会直接进行一次系统调用。这对于小的、频繁的读写操作效率非常低，因为系统调用本身有开销。
 * 缓存IO：
 *      1、BufReader: 在内存中维护一个缓冲区。当你从 BufReader 读取数据时，它会一次性从底层源（如文件）读取一大块数据（例如 8KB）到这个缓冲区中。
 *                    后续的读取请求会先从这个内存缓冲区中满足，直到缓冲区耗尽，才会再次进行系统调用去读取更多数据。
 *      2、BufWriter：在内存中维护一个缓冲区。当你向 BufWriter 写入数据时，数据首先被写入这个内存缓冲区。只有当缓冲区满了，或者你显式调用 flush 方法，或者 BufWriter 被丢弃（drop）时，
 *                    缓冲区中的所有数据才会被一次性通过系统调用刷新（flush）到底层目标（如文件）。
 */
// BufReader缓冲区
pub fn buf_reader_test() {
    let file = OpenOptions::new()
        .read(true)
        .open(PathBuf::from("src/chapter_4/inputTestFile.txt"))
        .unwrap();
    let mut buffer = BufReader::new(file);

    let mut content = String::new();
    buffer.seek_relative(12).unwrap();
    buffer.read_to_string(&mut content).unwrap();

    println!("{}", content);

    // 一行一行读
    buffer.seek_relative(0).unwrap();
    for item in buffer.lines() {
        match item {
            Ok(content) => println!("{}", content),
            Err(_) => println!("出现错误"),
        }
    }
}

// BufWriter缓存区
pub fn buf_writer_test() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(PathBuf::from("src/chapter_4/inputTestFile.txt"))
        .unwrap();
    let mut buffer = BufWriter::new(file);

    buffer.flush().unwrap();
    buffer.seek(std::io::SeekFrom::Start(100)).unwrap();
    let content = "\nReacter";
    buffer.write(content.as_bytes()).unwrap();
    buffer.flush().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::chapter_4::input_output::buf_reader_test;

    #[test]
    pub fn test_1() {
        buf_reader_test();
    }
}
