use std::{io::Error, path::PathBuf};

use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

// 提供了一种异步方法，用于将数据读取到缓冲区中，返回读取的字节数。
pub async fn async_read_file() -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .open(PathBuf::from("read.txt"))
        .await?;
    let mut capacity = [0; 1024];

    //  当 read（） 返回 Ok（0） 时，这意味着流已关闭。对 read（） 的任何进一步调用都将立即使用 Ok（0） 完成。
    let length = file.read(&mut capacity).await?;

    println!("{:?}", String::from_utf8(capacity[..length].to_vec()));
    Ok(())
}

// 读取所有字节知道结束EOF
pub async fn async_read_file_to_eof() -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .open(PathBuf::from("read.txt"))
        .await?;
    let mut buffer = Vec::new();

    let length = file.read_to_end(&mut buffer).await?;

    println!("{:?}, length = {length}", buffer);

    Ok(())
}

// 将缓冲区写入写入器，返回写入的字节数。
pub async fn async_write_file() -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(PathBuf::from("read.txt"))
        .await?;
    let content = "abcdef";

    let length = file.write(content.as_bytes()).await?;

    println!("length = {length}");
    Ok(())
}

// 将整个缓冲区写入编写器
pub async fn async_write_all_to_file() -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(PathBuf::from("read.txt"))
        .await?;
    let _ = file.write_all(b"abcdef").await?;

    Ok(())
}
