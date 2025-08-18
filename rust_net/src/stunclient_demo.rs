/**
 * stunclient = "0.4.1"
 *
 * stunclient = "0.4.1" 是一个用于 Rust 语言的库，它提供了一个简单的 STUN (Session Traversal Utilities for NAT) 客户端实现。
 * STUN 是一种网络协议（定义在 RFC 5389），主要用于帮助位于 NAT (Network Address Translation) 设备（如家庭路由器）后面的设备发现自己的公网地址和端口
 * 这对于实现 P2P (Peer-to-Peer) 通信至关重要，尤其是在 WebRTC、VoIP、在线游戏 等需要直接连接的应用中。当两个设备都在 NAT 后面时，它们需要知道对方的公网可达地址才能建立直接连接。
 *
 * stunclient 库的实际应用主要围绕着 NAT 穿透 (NAT Traversal) 和 发现公网可达地址 这一核心问题。它在需要建立 P2P (Peer-to-Peer) 连接或确定设备公网位置的场景中扮演着关键角色。
 */

#[cfg(test)]
mod stunclient_test {
    use std::net::{SocketAddr, UdpSocket};

    use stunclient::StunClient;

    // 查看自己的公网信息
    #[test]
    fn test_1() {
        // 1、创建一个UDP套接字，因为STUN 使用 UDP 协议。
        let socket = UdpSocket::bind("0.0.0.0:666").unwrap();
        // 2、指定一个 STUN 服务器的地址
        let stun_server: SocketAddr = "stun.voipbuster.com".parse().unwrap();

        // 3、创建Stun客户端
        let stun_client = StunClient::new(stun_server);

        // 4、发送 Binding 请求并获取响应
        let response = stun_client.query_external_address(&socket).unwrap();

        println!("ip = {}", response.ip());
        println!("port = {}", response.port());
        println!("{}-{}", response.is_ipv4(), response.is_ipv6());
    }
}
