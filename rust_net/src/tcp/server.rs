

/**
 * TcpServer不能放在测试代码中执行
 */

#[cfg(test)]
mod server_test{
    use std::{io::Read, net::{TcpListener, TcpStream}, thread};


    #[test]
    fn test(){
        tracing::info!("Tcp Server running...");
        let tcp_server = TcpListener::bind("0.0.0.0:8888").unwrap();

        // incoming会一直接受连接
        for tcp_stream in tcp_server.incoming() {
            match tcp_stream {
                Ok(stream) => {
                    tracing::info!("接收到连接");
                    thread::spawn(move || handle_connect(stream)).join().unwrap()
                },
                Err(e) => tracing::error!("{}", e)
            }
        }
    }

    fn handle_connect(mut stream: TcpStream) {
        let mut content_byte = [0; 1024];
        let length = stream.read(&mut content_byte).unwrap();
        let command = String::from_utf8(content_byte[..length].to_vec()).unwrap();
        println!("{}", command);
    }
}