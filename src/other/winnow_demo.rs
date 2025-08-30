/**
 * winnow = "0.7.13"
 *
 * Winnow是一个Rust解析器组合库，其token模块提供了多种用于解析和提取输入流中标记的函数。
 * 简单来说，它的作用是：帮助你轻松地将文本或二进制数据（输入流）转换成结构化的 Rust 数据（例如结构体、枚举、数组等）。
 *
 * 你可以把它想象成一套功能强大、可组合的“乐高积木”。
 * 你先用简单的积木（基础解析器）来解析一小部分数据（例如：一个数字、一个单词、一个特定的符号），
 * 然后将这些小积木组合成更复杂的积木（组合器），最终搭建出一个能解析整个复杂数据格式（如 JSON、XML、自定义协议、编程语言等）的完整解析器。
 *
 * 使用 winnow 通常分为三个步骤：1. 添加依赖 2. 构建解析器 3. 运行解析器。
 *
 * winnow提供的所有函数都是从头开始匹配，如果没有匹配成功就会报错。
 */
#[cfg(test)]
mod winnow_demo_test {

    use winnow::ascii::Caseless;
    use winnow::error::InputError;
    use winnow::{
        Parser,
        token::{any, literal, none_of, one_of, rest, rest_len, take, take_till, take_while},
    };

    #[test]
    fn any_test() {
        // any 函数用于匹配输入流中的一个标记。它是最基本的解析器之一，可以匹配任何类型的单个标记。
        let mut input = "abc";
        let result = any::<&str, InputError<&str>>
            .parse_next(&mut input)
            .unwrap();
        println!("匹配的字符: {}", result); // a
        println!("剩余输入: {}", input); // bc
    }

    #[test]
    fn literal_test() {
        /*
           literal 它会从输入的当前位置开始，一个字符一个字符地比对，看你给它的那个字符串是不是真的在那里。
           如果一模一样，就成功，把这段字符串“吃掉”（消耗掉）
           如果不一样，就失败，啥也不动
        */
        let mut input = "hello world";
        let result = literal::<Caseless<&str>, &str, InputError<&str>>(Caseless("hello"))
            .parse_next(&mut input)
            .unwrap();
        println!("匹配的字符: {}", result); // hello
        println!("剩余输入: {}", input); // world
    }

    #[test]
    fn none_of_test() {
        // none_of 函数用于匹配输入流中不在指定集合中的第一个字符。
        let mut input = "hello world";
        let result = none_of::<&str, &[char; 3], InputError<&str>>(&['e', 'w', 'd'])
            .parse_next(&mut input)
            .unwrap();
        println!("匹配的字符: {}", result); // h
        println!("剩余输入: {}", input); // ello world
    }

    #[test]
    fn one_of_test() {
        // one_of 函数用于匹配输入流中在指定集合中的第一个字符。
        let mut input = "hello world";
        let result =
            one_of::<&str, &[char; 3], InputError<&str>>(&['h', 'w', 'd']).parse_next(&mut input);
        match result {
            Ok(char) => {
                println!("匹配的字符: {}", char);
            }
            Err(err) => {
                println!("匹配失败: {:?}", err);
            }
        }
        println!("剩余输入: {}", input); // ello world
    }

    #[test]
    fn rest_test() {
        // rest 返回输入流中从当前位置开始的“所有剩余内容”，然后消耗掉它。
        let mut input = "hello world";
        let result = rest::<&str, InputError<&str>>
            .parse_next(&mut input)
            .unwrap();
        println!("匹配的字符: {}", result); // hello world
        println!("剩余输入: {}", input); // ""
    }

    #[test]
    fn rest_len_test() {
        // rest_len 函数用于获取输入流中剩余字符的长度。
        let mut input = "hello world";
        let result = rest_len::<&str, InputError<&str>>
            .parse_next(&mut input)
            .unwrap();
        println!("剩余字符长度: {}", result); // 11
    }

    #[test]
    fn take_test() {
        // take 函数用于匹配输入流中的指定数量的字符。
        let mut input = "hello world";
        let result = take::<usize, &str, InputError<&str>>(5_usize)
            .parse_next(&mut input)
            .unwrap();
        println!("匹配的字符: {}", result); // hello
        println!("剩余输入: {}", input); // world
    }

    #[test]
    fn take_till_test() {
        // take_till 从输入流的当前位置开始，持续“取出”并消耗所有字符，直到遇到第一个满足你指定条件的字符为止。
        let mut input = "hello world";

        let result = take_till::<_, &str, InputError<&str>>(0.., |item| item == ' ')
            .parse_next(&mut input)
            .unwrap();
        println!("匹配的字符: {}", result); // hello
        println!("剩余输入: {}", input); // world
    }

    #[test]
    fn take_unitl_test() {
        // take_until 从输入的当前位置开始，持续“取出”字符，不管它们是什么，直到遇到第一个满足你指定条件的字符为止。
        let mut input = "hello world";

        // 0..是一个 范围（Range）表达式，它表示从 0 到无穷大的范围。
        let result = take_till::<_, &str, InputError<&str>>(0.., |item| item == ' ')
            .parse_next(&mut input)
            .unwrap();
        println!("匹配的字符: {}", result); // hello
        println!("剩余输入: {}", input); // world
    }

    #[test]
    fn take_while_test() {
        /* take_while的作用：
           从输入的当前位置开始，一个字符一个字符地检查，如果这个字符满足你指定的条件，就“拿走”它（消耗输入）；继续看下一个。
           一旦遇到一个字符不满足条件，就立刻停止，把所有“拿走”的字符打包返回
        */
        let mut input = "hello world";

        let result = take_while::<_, &str, InputError<&str>>(0.., |item| item == ' ')
            .parse_next(&mut input)
            .unwrap();
        println!("匹配的字符: {}", result); // ''
        println!("剩余输入: {}", input); // 'hello world'
    }
}
