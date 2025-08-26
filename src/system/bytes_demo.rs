/**
 * bytes = "1.10.1"
 * Serde 支持是可选的，默认禁用。要启用，请使用特性 serde 。
 * 
 * bytes 是一个非常流行且重要的 Rust crate，它提供了用于高效处理字节数据的核心类型和工具。它主要解决了在处理网络 I/O、文件读写、序列化/反序列化等场景中频繁操作字节数组时的性能和内存管理问题。
 * 优点：
 *      1、性能: Bytes 的零拷贝切片对于处理大量数据流（如 HTTP 请求/响应、数据库查询结果、文件块）至关重要，避免了不必要的内存分配和复制。
 *      2、安全性: 在 Rust 的所有权和借用规则下，bytes crate 提供了安全且高效的内存管理抽象。
 *      3、生态整合: bytes 是许多高性能 Rust 网络库（如 tokio, hyper, tonic, warp）的基础依赖。这些库的输入/输出通常使用 Bytes 或 BytesMut。
 *      4、序列化/反序列化: 与 serde 集成，bytes 可以直接用于序列化和反序列化，无需额外的转换步骤。
 *      5、零拷贝: 提供了零拷贝的切片类型（如 Bytes 和 BytesMut），可以直接在网络 I/O 中使用，避免了数据复制。
 * 主要类型：
 *      1、Bytes: 不可变的、引用计数的字节容器，用于存储和操作字节数据。
 *      2、BytesMut: 可变的字节缓冲区，用于在不分配新内存的情况下修改字节数据。
 *      3、Buf: 读取 trait，定义了从字节容器中读取数据的方法。
 *      4、BufMut: 写入 trait，定义了向字节容器中写入数据的方法。
 * Bytes 和 BytesMut 都设计为零拷贝操作，但它们的零拷贝机制和适用场景有所不同。
 * 
 * “零拷贝”主要指的是 slice 操作。当你对一个 Bytes 实例进行切片（slice(start..end)）时，底层的字节数据本身不会被复制。
 * 执行clone()方法时，会增加引用计数，不会复制数据。
 */
#[cfg(test)]
mod bytes_test{
    use std::io::Read;

    use bytes::{BufMut, Bytes, BytesMut};


    #[test]
    fn test_1(){
        let data = vec![1,2,3,4,5,6];
        // 使用 Bytes (不可变，零拷贝)
        let bytes = Bytes::from(data);
        println!("{:?}", bytes);    // b"\x01\x02\x03\x04\x05\x06"

        // // 零拷贝切片
        println!("slice_1={:?}", &bytes[0..3]);    // b"\x01\x02\x03"
        println!("slice_2={:?}", &bytes[3..6]);    // b"\x04\x05\x06"
    }

    #[test]
    fn test_2(){
        // BytesMut (可变缓冲区)
        let mut bytes = BytesMut::with_capacity(1024);
        bytes.put_bytes('a' as u8, 1025); // a * 1025
        println!("{:?}", bytes.len());  // 1025

        bytes.put_slice(b"hello");
        bytes.put_f64(64_f64);
        bytes.put_f32(32_f32);
        println!("{:?}", bytes);
    }


    // 分块读取文件
    #[test]
    fn test_3(){
        let mut file = std::fs::File::open("src/system/bytes_demo.rs").unwrap();
        let mut chunks = Vec::new();

        let mut buffer = BytesMut::with_capacity(1024);

        let mut stop = 1;
        while stop != 0 {
            let mut temp_buffer = vec![0; 1024];
            stop = file.read(&mut temp_buffer).unwrap();
            if stop == 0 {
                break;
            }
            buffer.extend_from_slice(&temp_buffer[..stop]); // // 数据追加到连续缓冲区
            chunks.push(buffer.split().freeze());   // 零拷贝创建Bytes

            // 不好的做法：会产生大量小内存块
            // chunks.push(Bytes::from(temp_buffer[..stop].to_vec()));  // 存在内存拷贝
        }
        println!("{:?}", chunks);
    }
}