/**
 * mac_address = "1.1.8"
 * 
 * 提供了一种跨平台的方法来获取网络硬件的 MAC 地址。
 */
#[cfg(test)]
mod mac_address_demo {
    use mac_address::get_mac_address;


    #[test]
    fn test(){
        let mac_address = get_mac_address().unwrap();
        println!("mac_address: {:?}", mac_address);
    }
}