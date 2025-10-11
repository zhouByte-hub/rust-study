/**
 * strfmt = "0.2.5"
 * 
 * 用于格式化动态字符串的 rust 库
 */
#[cfg(test)]
mod string_format_test {
    use std::collections::HashMap;

    use strfmt::strfmt;


    #[test]
    fn test() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "zhangsan".to_string());
        vars.insert("age".to_string(), "12".to_string());
        vars.insert("address".to_string(), "beijing".to_string());

        let value = "name: {name}, age: {age}, address: {address}".to_string();
        let formatted = strfmt(&value, &vars).unwrap();
    
        println!("{}", formatted);
    }
}