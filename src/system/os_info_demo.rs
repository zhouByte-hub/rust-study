/**
 * os_info = "3.12.0"
 * 
 * 一个用于在运行时检测当前操作系统类型、版本和详细信息的实用工具。
 */

#[cfg(test)]
mod os_test{

    #[test]
    fn test(){
        let info = os_info::get();

        println!("Type = {}", info.os_type());
        println!("Version = {}", info.version());
        println!("Bitness = {}", info.bitness());
        println!("Architecture = {:?}", info.architecture());
    }
}