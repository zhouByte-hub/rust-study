use std::path::PathBuf;
use tokio::io::Error;

// Tokio 任务是一个异步的绿色线程，它们通过 tokio::spawn 进行创建，该函数会返回一个 JoinHandle 类型的句柄，调用者可以使用该句柄跟创建的任务进行交互
pub async fn download(path: String) -> Result<PathBuf, Error> {
    let result = tokio::spawn( async {
        // 模拟下载操作代码省略...
        PathBuf::from(path)
    });
    Ok(result.await?)
}



