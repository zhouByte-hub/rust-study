/**
 * ipnet = "2.11.0"
 *
 * 用于处理 IPv4 和 IPv6 网络地址（通常称为 IP 前缀）的类型和实用方法。
 * 新的`IpNet`、`Ipv4Net`和`Ipv6Net`类型基于 Rust 标准库中已有的`IpAddr`、`Ipv4Addr`和`Ipv6Addr`类型构建，并与其设计保持一致以维持一致性。
 * 该模块还提供了有用的特性，扩展了`Ipv4Addr`和`Ipv6Addr`，增加了用于`Add`、`Sub`、`BitAnd`和`BitOr`操作的 方法。
 * 该模块仅使用稳定特性，因此使用稳定工具链编译时保证能通过。
 */

#[cfg(test)]
mod ipnet_test {

    use ipnet::{IpNet, Ipv4Net}; // Ipv6Net

    #[test]
    fn test_1() {
        let v4: Ipv4Net = "192.168.1.101/24".parse().expect("无效的 IPv4 网段");
        // let _v6: Ipv6Net = "192.168.1.101/24".parse().unwrap();
        let _ip: IpNet = "192.168.1.101/24".parse().unwrap();

        println!("网络地址：{}", v4.network());
        println!("子网掩码：{}", v4.netmask());
        println!("广播地址：{}", v4.broadcast());
        println!("前缀长度：{}", v4.prefix_len());
    }

    #[test]
    fn test_2() {
        let v4: Ipv4Net = "192.168.1.101/24".parse().expect("无效的 IPv4 网段");
        // 判断一个两个IP是否在同一个子网
        let temp: Ipv4Net = "192.168.1.220/24".parse().expect("无效的IPv4网段");

        if v4.contains(&temp) {
            println!("在同一个子网中");
        } else {
            println!("不在同一个子网");
        }
    }

    #[test]
    fn test_3() {
        // 遍历子网中所有的IP，(谨慎使用大网段)
        let net: Ipv4Net = "192.168.1.0/30".parse().unwrap();
        for ip in net.hosts() {
            println!("{}", ip);
        }
    }

    #[test]
    fn test_4() {
        // 子网划分：将一个大子网划分成多个小子网
        let net: Ipv4Net = "192.168.1.0/24".parse().unwrap();
        let subnet = net.subnets(26).unwrap();

        for sub in subnet {
            println!("{}", sub);
        }
    }
}
