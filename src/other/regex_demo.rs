/**
 * regex = "1.11.2"
 * 
 * regex 是 Rust 编程语言中一个非常流行的用于处理正则表达式的库。
 * 它提供了强大的正则表达式功能，允许开发人员在 Rust 中进行复杂的字符串匹配和操作。
 */

#[cfg(test)]
mod regex_test{
    use regex::Regex;


    #[test]
    fn test_1(){
        // 创建一个正则对象， r 表示原始字符串字面量
        let regex = Regex::new(r"\d+").unwrap();

        assert!(!regex.is_match("abcdef"));
        assert!(regex.is_match("123123"));
    }

    /**
     * (?P<name>pattern)：创建一个命名捕获组，name 是捕获组的名称，pattern 是要匹配的模式。P 是 "Python" 的缩写，因为这种语法源自 Python 的正则表达式
     * ?P 组合在一起表示"命名捕获组"
     */
    #[test]
    fn test_2() {
        // 创建一个正则表达式对象，用于匹配日期格式 YYYY-MM-DD
        // (?x) - 启用注释模式，允许在正则表达式中添加空白和注释
        // (?P<year>\d{4}) - 命名捕获组"year"，匹配4位数字（年份）
        // (?P<month>\d{2}) - 命名捕获组"month"，匹配2位数字（月份）
        // (?P<day>\d{2}) - 命名捕获组"day"，匹配2位数字（日期）
        let regex = Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
        
        // 使用captures方法从字符串中提取匹配的捕获组
        // 输入字符串"2025-08-26"会被解析为：year=2025, month=08, day=26
        let caps = regex.captures("2025-08-26").unwrap();
        println!("{:?}", &caps["year"]);
        println!("{:?}", &caps["month"]);
        println!("{:?}", &caps["day"]);
    }

    #[test]
    fn test_3(){
        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let hay = "On 2025-08-26, the date is formatted as YYYY-MM-DD. On 2026-08-26 successful";

        // 避免在循环中编译正则表达式，因为编译的成本较高
        for (year, month, day) in regex.captures_iter(hay).map(|c| (
            c.get(1).unwrap().as_str(),
            c.get(2).unwrap().as_str(),
            c.get(3).unwrap().as_str(),
        )) {
            println!("{}-{}-{}", year, month, day);
        }
    }
}