/**
 * sys-locale = "0.3.2"
 *
 * 一个轻量级库，用于获取当前系统区域设置
 */
#[cfg(test)]
mod locale_test {
    use sys_locale::get_locale;

    #[test]
    fn test() {
        let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
        println!("local = {}", locale); // zh-CN
    }
}
