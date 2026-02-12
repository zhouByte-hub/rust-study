/**
 * ssh2 = "0.9.5"
 * 
 * 用于与 SSH 服务器交互、执行远程命令、转发本地端口等的 libssh2 绑定。
 */

#[cfg(test)]
mod ssh_test{
    use std::{io::Write, net::TcpStream};



    #[test]
    fn test() {
        let mut connect = TcpStream::connect("127.0.0.1:22").unwrap();
        connect.write("abcdef".as_bytes()).unwrap();
    }
}