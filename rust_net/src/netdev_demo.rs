/**
 * netdev = "0.37.1"
 *
 * 一个用于 获取系统网络接口（Network Interfaces）信息 的 Rust crate。
 * 它可以帮助你在程序中列出本机的网卡、获取 IP 地址、MAC 地址、状态（启用/禁用）、支持的协议等信息。
 *
 * 这个库适用于：
 *      1、网络诊断工具
 *      2、服务发现
 *      3、显示本地网络配置
 *      4、绑定服务器到特定网卡
 *      5、构建网络监控程序
 *
 * netdev常常与ipnetwork或者ipnet一起使用。
 */

#[cfg(test)]
mod netdev_test {

    #[test]
    fn test_1() {
        // 获取机器上所有网络接口
        let interfaces = netdev::get_interfaces();
        for item in interfaces.iter() {
            println!("接口名称：{}", item.name);
            println!("接口描述：{:?}", item.description);
            println!("接口状态：{}", item.flags);
            println!("MAC地址: {}", item.mac_addr.unwrap());
            println!("是否是广播地址:{}", item.is_broadcast());
            println!("是否是多播地址:{}", item.is_multicast());
            println!("是否是本地回环地址:{}", item.is_loopback());
            println!("是否是物理地址:{}", item.is_physical());
            println!("是否是运行状态：{}", item.is_running());
            println!("是否是隧道：{}", item.is_tun());
            println!("接口是否启动：{}", item.is_up());

            // ipv4地址
            for ip in item.ipv4_addrs().iter() {
                println!("ipv4地址: {:?}", ip);
            }

            // ipv6地址
            for ip in item.ipv6_addrs().iter() {
                println!("ipv6地址: {:?}", ip);
            }
            println!("==================================================")
        }
    }
}
