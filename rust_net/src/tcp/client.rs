/**
 * TcpClient不能放在测试代码中执行
 */

#[cfg(test)]
mod client_test {
    use std::{io::Write, net::TcpStream, time::Duration};

    #[test]
    fn test() {
        tracing::info!("Tcp Client running...");
        // 这部分代码只会建立一次连接，当这个连接任务执行完毕后就会断开连接。
        let tcp_stream = TcpStream::connect("127.0.0.1:8888");
        match tcp_stream {
            Ok(mut stream) => {
                tracing::info!("成功建立连接...");
                stream
                    .set_read_timeout(Some(Duration::from_secs(10)))
                    .unwrap();
                stream
                    .set_write_timeout(Some(Duration::from_secs(10)))
                    .unwrap();

                let mut commond = String::new();
                let length = std::io::stdin().read_line(&mut commond).unwrap();
                if length == 0 {
                    println!("请输入命令");
                }
                stream.write(&commond.as_bytes()).unwrap();
            }
            Err(e) => tracing::error!("{e}"),
        }
    }
}
