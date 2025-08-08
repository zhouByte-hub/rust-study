
#[cfg(test)]
mod test {
    use std::{io::Result, net::UdpSocket};


    #[test]
    fn test() -> Result<()>{
        let udp_client = UdpSocket::bind("0.0.0.0:8890")?;
        udp_client.connect("127.0.0.1:8889")?;

        let mut input = String::new();
        let length = std::io::stdin().read_line(&mut input)?;
        if length <= 0 {
            tracing::error!("请输入指令");
        }
        udp_client.send(input.as_bytes())?; // 因为上面使用了connect进行了连接，所以这里可以使用send发送内容，而不适用send_to

        // 接受响应
        if input == "info" {
            let mut info = [0; 255];
            let (length, src) = udp_client.recv_from(&mut info)?;
            tracing::info!("接收到{}发送的响应，内容长度：{}, 内容为：{:?}", src, length, String::from_utf8(info[..length].to_vec()));
        }

        Ok(())
    }
}