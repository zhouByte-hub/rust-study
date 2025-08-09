use std::{path::PathBuf, vec};

use tokio::{
    fs::OpenOptions,
    io::{AsyncBufReadExt, BufReader, Error},
};
use tokio_stream::StreamExt;

/**
 * 流是异步值序列。它可以被认为是标准库的迭代器特征的异步版本。
 */
pub async fn async_stream() -> Result<(), Error> {
    let list = vec![1, 2, 3, 4, 5, 6];
    let mut iter = tokio_stream::iter(list);

    while let Some(next) = iter.next().await {
        println!("{next}");
    }
    println!("{:?}", iter); // 所有权没有被转移
    Ok(())
}

pub async fn file_stream() -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .open(PathBuf::from("read.txt"))
        .await?;

    let mut buffer = BufReader::new(&mut file).lines();
    while let Some(next) = buffer.next_line().await? {
        println!("{}", next);
    }
    Ok(())
}

pub async fn map_stream() -> Result<(), Error> {
    let list = vec![1, 2, 3, 4, 5];
    let mut iter = tokio_stream::iter(list).map(|x| x + 1);

    while let Some(next) = iter.next().await {
        println!("{}", next);
    }
    Ok(())
}

pub async fn filter_stream() -> Result<(), Error> {
    let list = vec![1, 2, 3, 4, 5];
    let mut iter = tokio_stream::iter(list).filter(|x| *x % 2 == 0);

    while let Some(next) = iter.next().await {
        println!("{}", next);
    }

    Ok(())
}

pub async fn take_stream() {
    let list = vec![1, 2, 3, 4, 5];
    // 使用take来限制元素的输出数量，只输出前三个
    let mut iter = tokio_stream::iter(list).take(3);

    while let Some(next) = iter.next().await {
        println!("{}", next);
    }
}

pub async fn fold_stream() {
    let list = vec![1, 2, 3, 4, 5];
    // 可以使用async move来让闭包异步化 |acc, x| async move {}
    let sum = tokio_stream::iter(list).fold(0, |acc, x| acc + x).await;
    println!("{}", sum);
}
