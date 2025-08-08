
#[cfg(test)]
mod test {
    use std::{io::Result, net::UdpSocket};


    /**
     * UPD 一次只能处理一个连接，需要配合loop循环不断接受连接
     */
    #[test]
    fn test() -> Result<()>{
        tracing::info!("UPD Server running...");
        let upd_server = UdpSocket::bind("0.0.0.0:8889")?;
        
        // 接受连接并从连接中获取内容
        let mut content = [0, 255];
        let (length, connect_src) = upd_server.recv_from(&mut content)?;

        let command = String::from_utf8(content[..length].to_vec()).unwrap();
        println!("{command}");

        // 响应连接
        if command == "info" {
            let info = String::from("version: 1.0.1");
            upd_server.send_to(info.as_bytes(), connect_src)?;
        }

        Ok(())
    }
}