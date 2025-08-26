/**
 * wildmatch = "2.4.0"
 *
 * 一个用于 Rust 语言的轻量级、快速的通配符匹配（Wildcard Matching）库
 * 它允许你使用类似 shell 命令行中常见的通配符模式（如 *.txt, file?.dat）来匹配字符串。这种匹配方式比正则表达式更简单、更高效，适用于文件名匹配、路径过滤、简单的字符串模式匹配等场景。
 * *：匹配多个字符(0~n)
 * ？：匹配单个字符(1)
 */
#[cfg(test)]
mod wild_test {
    use wildmatch::WildMatch;

    #[test]
    fn test_1() {
        let mut pattern = WildMatch::new("hello*.txt");

        println!("{}", pattern.matches("hello world.txt")); // true
        println!("{}", pattern.matches("input")); // false
        println!("{}", pattern.matches("hellow.txt")); // true
        println!("{}", pattern.matches("hello.txt")); // true

        pattern = WildMatch::new("wild?.rs");
        println!("{}", pattern.matches("wildmatch.rs")); // false
        println!("{}", pattern.matches("wild1.rs")); // true
        println!("{}", pattern.matches("wild.rs")); // false
    }
}
