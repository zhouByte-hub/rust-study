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
mod winnow_token_test {

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

#[cfg(test)]
mod winnow_combinator_test {
    use std::collections::HashMap;

    use winnow::Result;
    use winnow::ascii::{alpha0, alpha1, digit0, digit1};
    use winnow::combinator::{
        alt, cond, delimited, fill, iterator, not, opt, peek, permutation, preceded, repeat,
        repeat_till, separated, seq, terminated,
    };
    use winnow::error::ContextError;
    use winnow::prelude::*;
    use winnow::token::{any, literal, take_while};

    #[derive(Debug, Eq, PartialEq)]
    pub(crate) struct Color {
        pub(crate) red: u8,
        pub(crate) green: u8,
        pub(crate) blue: u8,
    }

    /**
     * 按顺序应用多个解析器，并将结果组合成一个结构体或元组。
     */
    #[test]
    pub fn seq_test() {
        let mut input = "#a1b2c3";
        // 一定要声明出错误类型：winnow::error::ContextError
        let result: Result<Color, ContextError> = seq!(Color {
            _: '#',
            red: hex_primary,
            green: hex_primary,
            blue: hex_primary
        })
        .parse_next(&mut input);
        match result {
            Ok(color) => {
                println!("{:?}", color);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    fn hex_primary(input: &mut &str) -> Result<u8> {
        take_while(2, |c: char| c.is_ascii_hexdigit())
            .try_map(|input| u8::from_str_radix(input, 16))
            .parse_next(input)
    }

    /**
     * 尝试按顺序应用多个解析器，返回第一个成功的解析器的结果。
     */
    #[test]
    fn alt_test() {
        let mut input = "hello world";
        let result: Result<&str, ContextError> =
            alt((literal("hello"), literal("world"))).parse_next(&mut input);
        match result {
            Ok(str) => {
                println!("{}", str);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
        println!("剩余输入: {}", input);
    }

    /**
     * 使解析器可选，即使解析失败也不会报错，返回 None。
     */
    #[test]
    fn opt_test() {
        let mut input = "hello world";
        let result: Result<Option<&'static str>, ContextError> =
            opt(literal("abc")).parse_next(&mut input);
        match result {
            Ok(Some(str)) => {
                println!("{}", str);
            }
            Ok(None) => {
                println!("None");
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    /**
     * 重复应用解析器，返回所有成功的结果，之间用指定的分隔符隔开。
     */
    #[test]
    fn separated_test() {
        let mut input = "hello,hello,hello";
        let result: Result<Vec<&'static str>, ContextError> =
            separated(0.., literal("hello"), ",").parse_next(&mut input);
        match result {
            Ok(vec) => {
                for item in vec {
                    println!("{}", item);
                }
            }
            Err(err) => println!("{:?}", err),
        }
    }

    /**
     * 重复应用解析器，返回所有成功的结果。
     */
    #[test]
    fn repeat_test() {
        let mut input = "abcabcabc";
        let result: Result<Vec<&str>, ContextError> =
            repeat(0.., literal("abc")).parse_next(&mut input);
        match result {
            Ok(vec) => {
                for item in vec {
                    println!("{}", item);
                }
            }
            Err(err) => println!("{:?}", err),
        }
        println!("剩余输入: {}", input);
    }

    /**
     * 重复应用解析器，直到遇到指定的分隔符，返回所有成功的结果。
     */
    #[test]
    fn repeat_till_test() {
        let mut input = "abcabcabcend";
        let result: Result<(Vec<&str>, &str), ContextError> =
            repeat_till(1.., literal("abc"), literal("end")).parse_next(&mut input);
        match result {
            // vec是结果，str 是停止符
            Ok((vec, str)) => {
                for item in vec {
                    println!("{}", item);
                }
                println!("{}", str);
            }
            Err(err) => println!("{:?}", err),
        }
        println!("剩余输入: {}", input);
    }

    /**
     * 将解析器的结果应用一个函数进行转换。
     */
    #[test]
    fn map_test() {
        let mut input = "123123";
        let result: Result<usize, ContextError> = take_while(0.., |c: char| c.is_ascii_digit())
            .map(|s: &'static str| s.len())
            .parse_next(&mut input);
        match result {
            Ok(len) => {
                println!("{}", len);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    /**
     * 尝试将解析器的结果应用一个函数进行转换，如果转换失败则返回错误。
     */
    #[test]
    fn try_map_test() {
        let mut input = "123123";

        let result: Result<u8, ContextError> = any
            .try_map(|item: char| {
                // 将字符转换为字符串切片，然后解析为 u8
                item.to_string().parse::<u8>()
            })
            .parse_next(&mut input);

        match result {
            Ok(num) => {
                println!("{}", num);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    /**
     * 验证解析器的结果是否满足条件，如果不满足则返回错误。
     */
    #[test]
    fn verify_test() {
        let mut input = "123123";
        let result: Result<char, ContextError> = any
            .verify(|c: &char| c.is_ascii_digit())
            .parse_next(&mut input);
        match result {
            Ok(c) => println!("{}", c),
            Err(err) => println!("{:?}", err),
        }
    }

    /**
     * 按顺序应用多个解析器，返回所有成功的结果。
     * 如果所有子解析器都成功，Permutation 将会成功。它以解析器元组为参数，并返回解析器结果的元组。
     */
    #[test]
    fn permutation_test() {
        let mut input = "123abc";
        let result: Result<(&str, &str), ContextError> =
            permutation((digit0, alpha0)).parse_next(&mut input);
        match result {
            Ok((digit, alpha)) => {
                println!("{}", digit);
                println!("{}", alpha);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    /**
     * 条件解析器，根据条件是否满足来选择不同的解析器。
     */
    #[test]
    fn condition_test() {
        let mut input = "123abc";
        let result: Result<Option<&str>, ContextError> =
            cond(!input.is_empty(), digit0).parse_next(&mut input);
        match result {
            Ok(Some(digit)) => {
                println!("{:?}", digit);
            }
            Ok(None) => {
                println!("None");
            }
            Err(err) => println!("{:?}", err),
        }
    }

    /**
     * 解析器的结果必须被包裹在指定的分隔符之间。
     */
    #[test]
    fn delimited_test() {
        let mut input = "123abc";
        let result: Result<&str, ContextError> =
            delimited("12", digit0, "abc").parse_next(&mut input);
        match result {
            Ok(digit) => {
                println!("{:?}", digit);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    /**
     * 填充解析器的结果到一个固定大小的数组中。
     */
    #[test]
    fn fill_test() {
        let mut input = "123_abc";
        let mut buf = [""; 5];
        let result: Result<(), ContextError> = fill(digit0, &mut buf).parse_next(&mut input);
        match result {
            Ok(()) => {
                println!("{:?}", buf);
            }
            Err(err) => println!("{:?}", err),
        }
        println!("剩余输入: {}", input); // _abc
    }

    /**
     * 解析一个模式，然后解析另一个模式，只返回第一个模式的结果
     */
    #[test]
    fn terminated_test() {
        let mut input = "123_abc";
        let result: Result<&str, ContextError> =
            terminated(digit0, literal("_")).parse_next(&mut input);
        match result {
            Ok(digit) => {
                println!("{:?}", digit);
            }
            Err(err) => println!("{:?}", err),
        }
        println!("剩余输入: {}", input); // abc
    }

    /**
     * 重复执行内嵌解析器，惰性返回结果
     * 如果成功，调用迭代器的 ParserIterator::finish 方法获取剩余输入；如果遇到错误，则返回错误值。
     */
    #[test]
    fn iterator_test() {
        let input = "123|234|345|456";
        let mut iter = iterator(input, terminated(digit1, "|"));

        let parsed = iter.map(|v| (v, v.len())).collect::<HashMap<_, _>>();
        let _: ModalResult<_> = iter.finish();

        parsed.iter().for_each(|(key, value)| {
            println!("{}-{}", key, value);
        });
    }

    /**
     * 如果子解析器返回错误，则成功。
     * 这不会前进 Stream
     */
    #[test]
    fn not_test() {
        let mut input = "abc";
        let result: Result<(), ContextError> = not(digit1).parse_next(&mut input);
        match result {
            Ok(()) => println!("成功"),
            Err(err) => println!("{:?}", err),
        }
        println!("剩余输入: {}", input);
    }

    /**
     * 查看解析器的结果，不消耗输入。
     */
    #[test]
    fn peek_test() {
        let mut input = "abcdef";
        let result: Result<&str, ContextError> = peek(alpha1).parse_next(&mut input);
        match result {
            Ok(c) => println!("{}", c),
            Err(err) => println!("{:?}", err),
        }
        println!("剩余输入: {}", input);
    }

    /**
     * 顺序组合两个解析器，仅返回第二个解析器的输出。
     */
    #[test]
    fn preceded_test() {
        let mut input = "123abc";
        let result: Result<&str, ContextError> = preceded(digit1, alpha1).parse_next(&mut input);
        match result {
            Ok(c) => println!("{}", c),
            Err(err) => println!("{:?}", err),
        }
        println!("剩余输入: {}", input);
    }
}






/**
 * winnow::ascii 模块函数列表：
 * alpha0：识别零个或多个小写和大写ASCII字母字符：'a'..='z', 'A'..='Z'
 * alpha1：识别一个或多个小写和大写ASCII字母字符：'a'..='z', 'A'..='Z'
 * alphanumeric0：识别零个或多个ASCII数字和字母字符：'a'..='z', 'A'..='Z', '0'..='9'
 * alphanumeric1：识别一个或多个ASCII数字和字母字符：'a'..='z', 'A'..='Z', '0'..='9'
 * crlf：识别字符串 "\r\n"
 * dec_int：解码十进制有符号整数（例如 i32）
 * dec_uint：解码十进制无符号整数（例如 u32）
 * digit0：识别零个或多个ASCII数字字符：'0'..='9'
 * digit1：识别一个或多个ASCII数字字符：'0'..='9'
 * escaped：解析转义字符，并取消转义
 * escaped_transform：已弃用，被 escaped 替代
 * float：识别文本格式的浮点数并返回 f32 或 f64
 * hex_digit0：识别零个或多个ASCII十六进制数字字符：'0'..='9', 'A'..='F', 'a'..='f'
 * hex_digit1：识别一个或多个ASCII十六进制数字字符：'0'..='9', 'A'..='F', 'a'..='f'
 * hex_uint：解码可变宽度的十六进制整数（例如 u32）
 * line_ending：识别行结束（包括 "\n" 和 "\r\n"）
 * multispace0：识别零个或多个空格、制表符、回车和换行符
 * multispace1：识别一个或多个空格、制表符、回车和换行符
 * newline：匹配换行符 '\n'
 * oct_digit0：识别零个或多个八进制字符：'0'..='7'
 * oct_digit1：识别一个或多个八进制字符：'0'..='7'
 * space0：识别零个或多个空格和制表符
 * space1：识别一个或多个空格和制表符
 * tab：匹配制表符 '\t'
 * take_escaped：识别带有转义字符的输入片段
 * till_line_ending：识别直到 "\r\n"、"\n" 或文件结束的 0+ 个字符的字符串
 */
#[cfg(test)]
mod winnow_ascii_test {
    use winnow::error::ContextError;
    use winnow::{
        Parser,
        ascii::{alpha1, digit1, space0},
    };

    /**
     * 解析一个或多个 ASCII 数字字符。
     */
    #[test]
    fn digit1_test() {
        let mut input = "123abc";
        /*
           digit0：匹配 0 个到多个数字字符
           digit1: 匹配一个到多个数字字符
        */
        let result: Result<&str, ContextError> = digit1.parse_next(&mut input);
        match result {
            Ok(c) => println!("{}", c),
            Err(err) => println!("{:?}", err),
        }
    }

    /**
     * 解析一个或多个 ASCII 字母字符。
     */
    #[test]
    fn alpha1_test() {
        let mut input = "abc123";
        /*
           alpha0：匹配 0 个到多个字母字符
           alpha1: 匹配一个到多个字母字符
        */
        let result: Result<&str, ContextError> = alpha1.parse_next(&mut input);
        match result {
            Ok(c) => println!("{}", c),
            Err(err) => println!("{:?}", err),
        }
        println!("剩余输入: {}", input);
    }

    /**
     * 解析 0 个或多个空格字符。
     */
    #[test]
    fn space0_test() {
        let mut input = "   abc";
        /*
           space0：匹配 0 个到多个空格字符
           space1: 匹配一个到多个空格字符
        */
        let result: Result<&str, ContextError> = space0.parse_next(&mut input);
        match result {
            Ok(c) => println!("{}", c),
            Err(err) => println!("{:?}", err),
        }
        println!("剩余输入: {}", input);
    }
}
