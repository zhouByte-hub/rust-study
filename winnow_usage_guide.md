# Winnow 使用指南

## 简介

Winnow 是一个 Rust 的解析器组合器库，它使得解析变得轻而易举。它旨在成为你的"全能"解析器，类似于人们对待正则表达式的方式。Winnow 支持声明式编写解析器，同时不阻碍命令式风格的解析，作为一个开放式工具箱而非封闭式框架。它足够灵活，可用于任何应用程序，包括解析字符串、二进制数据或单独的词法分析和解析阶段。

## 核心概念

Winnow 的核心概念包括：

1. **解析器（Parser）**：一个函数，它接收输入并返回解析结果或错误
2. **组合器（Combinator）**：用于组合多个解析器的函数
3. **流（Stream）**：解析器操作的输入数据
4. **错误处理**：管理解析过程中出现的错误

## 模块概览

Winnow 提供了以下主要模块：

- `ascii`：字符特定的解析器和组合器
- `binary`：识别数字的解析器
- `combinator`：解析器和组合器列表
- `error`：错误管理
- `prelude`：可供全局导入的核心概念
- `stream`：组合器解析的流能力
- `token`：从流中提取标记的解析器
- `multi`：多次应用解析器的组合器
- `sequence`：序列解析相关的组合器
- `branch`：条件解析相关的组合器
- `bytes`：字节特定的解析器和组合器
- `character`：字符特定的解析器和组合器

## 主要函数和用法

### 1. token 模块

#### literal

**作用**：精确匹配并消耗一段文本或字节序列。

**示例**：
```rust
use winnow::{prelude::*, token::literal};

#[test]
fn literal_test() {
    let input = "hello world";
    let result = literal("hello").parse_next(input).unwrap();
    assert_eq!(result, "hello");
}
```

#### take_while

**作用**：当满足条件时持续获取字符或字节。

**示例**：
```rust
use winnow::{prelude::*, token::take_while};

#[test]
fn take_while_test() {
    let input = "123abc";
    let result = take_while(1.., |c: char| c.is_ascii_digit()).parse_next(input).unwrap();
    assert_eq!(result, "123");
}
```

### 2. combinator 模块

#### seq

**作用**：按顺序应用多个解析器，并将结果组合成一个结构体或元组。

**示例**：
```rust
use winnow::{combinator::seq, prelude::*, token::{take_while, literal}};

#[derive(Debug, Eq, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn hex_color(input: &mut &str) -> PResult<Color> {
    seq!(Color {
        _: literal('#'),
        red: hex_primary,
        green: hex_primary,
        blue: hex_primary
    })
    .parse_next(input)
}

fn hex_primary(input: &mut &str) -> PResult<u8> {
    take_while(2..=2, |c: char| c.is_ascii_hexdigit())
        .try_map(|s: &str| u8::from_str_radix(s, 16))
        .parse_next(input)
}

#[test]
fn seq_test() {
    let input = "#1a2b3c";
    let result = hex_color.parse_next(input).unwrap();
    assert_eq!(result, Color { red: 26, green: 43, blue: 60 });
}
```

#### alt

**作用**：尝试多个解析器，使用第一个成功的解析器的结果。

**示例**：
```rust
use winnow::{combinator::alt, prelude::*, token::literal};

#[test]
fn alt_test() {
    let input = "hello";
    let result = alt((literal("hello"), literal("world"))).parse_next(input).unwrap();
    assert_eq!(result, "hello");
}
```

#### opt

**作用**：使解析器可选，即使解析失败也返回成功（结果为 None）。

**示例**：
```rust
use winnow::{combinator::opt, prelude::*, token::literal};

#[test]
fn opt_test() {
    let input = "hello world";
    let result = opt(literal("hello")).parse_next(input).unwrap();
    assert_eq!(result, Some("hello"));
    
    let input2 = "world";
    let result2 = opt(literal("hello")).parse_next(input2).unwrap();
    assert_eq!(result2, None);
}
```

#### many0

**作用**：应用解析器零次或多次，收集所有结果到一个 Vec 中。

**示例**：
```rust
use winnow::{combinator::many0, prelude::*, token::literal};

#[test]
fn many0_test() {
    let input = "abcabcabc";
    let result = many0(literal("abc")).parse_next(input).unwrap();
    assert_eq!(result, vec!["abc", "abc", "abc"]);
}
```

#### many1

**作用**：应用解析器一次或多次，收集所有结果到一个 Vec 中。

**示例**：
```rust
use winnow::{combinator::many1, prelude::*, token::literal};

#[test]
fn many1_test() {
    let input = "abcabcabc";
    let result = many1(literal("abc")).parse_next(input).unwrap();
    assert_eq!(result, vec!["abc", "abc", "abc"]);
}
```

#### separated0

**作用**：应用解析器零次或多次，每次由分隔符分隔，收集所有结果到一个 Vec 中。

**示例**：
```rust
use winnow::{combinator::separated0, prelude::*, token::literal};

#[test]
fn separated0_test() {
    let input = "a,b,c";
    let result = separated0(literal("a"), literal(",")).parse_next(input).unwrap();
    assert_eq!(result, vec!["a", "a", "a"]);
}
```

#### separated1

**作用**：应用解析器一次或多次，每次由分隔符分隔，收集所有结果到一个 Vec 中。

**示例**：
```rust
use winnow::{combinator::separated1, prelude::*, token::literal};

#[test]
fn separated1_test() {
    let input = "a,b,c";
    let result = separated1(literal("a"), literal(",")).parse_next(input).unwrap();
    assert_eq!(result, vec!["a", "a", "a"]);
}
```

#### fold_repeat0

**作用**：应用解析器零次或多次，并使用累加函数组合结果。

**示例**：
```rust
use winnow::{combinator::fold_repeat0, prelude::*, token::literal};

#[test]
fn fold_repeat0_test() {
    let input = "abcabcabc";
    let result = fold_repeat0(
        literal("abc"), // 要重复的解析器
        || 0, // 初始值
        |acc, _| acc + 1 // 累加函数
    ).parse_next(input).unwrap();
    assert_eq!(result, 3);
}
```

#### fold_repeat1

**作用**：应用解析器一次或多次，并使用累加函数组合结果。

**示例**：
```rust
use winnow::{combinator::fold_repeat1, prelude::*, token::literal};

#[test]
fn fold_repeat1_test() {
    let input = "abcabcabc";
    let result = fold_repeat1(
        literal("abc"), // 要重复的解析器
        || 0, // 初始值
        |acc, _| acc + 1 // 累加函数
    ).parse_next(input).unwrap();
    assert_eq!(result, 3);
}
```

#### map

**作用**：将解析器的结果应用一个函数进行转换。

**示例**：
```rust
use winnow::{combinator::map, prelude::*, token::take_while};

#[test]
fn map_test() {
    let input = "123";
    let result = map(
        take_while(1.., |c: char| c.is_ascii_digit()),
        |s: &str| s.parse::<i32>().unwrap()
    ).parse_next(input).unwrap();
    assert_eq!(result, 123);
}
```

#### flat_map

**作用**：将解析器的结果应用一个函数，该函数返回另一个解析器。

**示例**：
```rust
use winnow::{combinator::flat_map, prelude::*, token::{take_while, literal}};

#[test]
fn flat_map_test() {
    let input = "3abc";
    let result = flat_map(
        take_while(1.., |c: char| c.is_ascii_digit()).try_map(|s: &str| s.parse::<usize>()),
        |n| take(n..=n)
    ).parse_next(input).unwrap();
    assert_eq!(result, "abc");
}
```

#### try_map

**作用**：将解析器的结果应用一个可能失败的函数进行转换。

**示例**：
```rust
use winnow::{combinator::try_map, prelude::*, token::take_while};

#[test]
fn try_map_test() {
    let input = "123";
    let result = try_map(
        take_while(1.., |c: char| c.is_ascii_digit()),
        |s: &str| s.parse::<i32>()
    ).parse_next(input).unwrap();
    assert_eq!(result, 123);
}
```

#### verify

**作用**：验证解析器的结果是否满足条件，如果不满足则返回错误。

**示例**：
```rust
use winnow::{combinator::verify, prelude::*, token::take_while};

#[test]
fn verify_test() {
    let input = "123";
    let result = verify(
        take_while(1.., |c: char| c.is_ascii_digit()),
        |s: &str| s.len() == 3
    ).parse_next(input).unwrap();
    assert_eq!(result, "123");
}
```

### 3. ascii 模块

#### digit1

**作用**：解析一个或多个 ASCII 数字字符。

**示例**：
```rust
use winnow::{ascii::digit1, prelude::*};

#[test]
fn digit1_test() {
    let input = "123abc";
    let result = digit1.parse_next(input).unwrap();
    assert_eq!(result, "123");
}
```

#### alpha1

**作用**：解析一个或多个 ASCII 字母字符。

**示例**：
```rust
use winnow::{ascii::alpha1, prelude::*};

#[test]
fn alpha1_test() {
    let input = "abc123";
    let result = alpha1.parse_next(input).unwrap();
    assert_eq!(result, "abc");
}
```

#### space0

**作用**：解析零个或多个 ASCII 空白字符。

**示例**：
```rust
use winnow::{ascii::space0, prelude::*};

#[test]
fn space0_test() {
    let input = "   abc";
    let result = space0.parse_next(input).unwrap();
    assert_eq!(result, "   ");
}
```

#### space1

**作用**：解析一个或多个 ASCII 空白字符。

**示例**：
```rust
use winnow::{ascii::space1, prelude::*};

#[test]
fn space1_test() {
    let input = "   abc";
    let result = space1.parse_next(input).unwrap();
    assert_eq!(result, "   ");
}
```

### 4. binary 模块

#### be_u8

**作用**：解析一个大端序的 8 位无符号整数。

**示例**：
```rust
use winnow::{binary::be_u8, prelude::*};

#[test]
fn be_u8_test() {
    let input = &[0x12, 0x34, 0x56];
    let result = be_u8.parse_next(input).unwrap();
    assert_eq!(result, 0x12);
}
```

#### be_u16

**作用**：解析一个大端序的 16 位无符号整数。

**示例**：
```rust
use winnow::{binary::be_u16, prelude::*};

#[test]
fn be_u16_test() {
    let input = &[0x12, 0x34, 0x56];
    let result = be_u16.parse_next(input).unwrap();
    assert_eq!(result, 0x1234);
}
```

#### be_u32

**作用**：解析一个大端序的 32 位无符号整数。

**示例**：
```rust
use winnow::{binary::be_u32, prelude::*};

#[test]
fn be_u32_test() {
    let input = &[0x12, 0x34, 0x56, 0x78, 0x9a];
    let result = be_u32.parse_next(input).unwrap();
    assert_eq!(result, 0x12345678);
}
```

#### le_u8

**作用**：解析一个小端序的 8 位无符号整数。

**示例**：
```rust
use winnow::{binary::le_u8, prelude::*};

#[test]
fn le_u8_test() {
    let input = &[0x12, 0x34, 0x56];
    let result = le_u8.parse_next(input).unwrap();
    assert_eq!(result, 0x12);
}
```

#### le_u16

**作用**：解析一个小端序的 16 位无符号整数。

**示例**：
```rust
use winnow::{binary::le_u16, prelude::*};

#[test]
fn le_u16_test() {
    let input = &[0x12, 0x34, 0x56];
    let result = le_u16.parse_next(input).unwrap();
    assert_eq!(result, 0x3412);
}
```

#### le_u32

**作用**：解析一个小端序的 32 位无符号整数。

**示例**：
```rust
use winnow::{binary::le_u32, prelude::*};

#[test]
fn le_u32_test() {
    let input = &[0x12, 0x34, 0x56, 0x78, 0x9a];
    let result = le_u32.parse_next(input).unwrap();
    assert_eq!(result, 0x78563412);
}
```

### 5. stream 模块

#### Stream trait

**作用**：提供流处理的基本能力，允许解析器操作不同类型的输入。

**示例**：
```rust
use winnow::{stream::Stream, prelude::*};

#[test]
fn stream_test() {
    let input = "hello world";
    let mut stream = input;
    
    // 获取流的第一个字符
    let first_char = stream.next_token();
    assert_eq!(first_char, Some('h'));
    
    // 获取流的剩余部分
    let remaining = stream.finish();
    assert_eq!(remaining, "ello world");
}
```

### 6. error 模块

#### ParseError

**作用**：表示解析过程中发生的错误。

**示例**：
```rust
use winnow::{error::ParseError, prelude::*, token::literal};

#[test]
fn error_test() {
    let input = "world";
    let result = literal("hello").parse_next(input);
    
    match result {
        Ok(_) => println!("解析成功"),
        Err(e) => println!("解析失败: {:?}", e),
    }
}
```

#### ErrorKind

**作用**：表示错误的类型，用于更精确的错误分类。

**示例**：
```rust
use winnow::{error::{ErrorKind, ParseError}, prelude::*, token::literal};

#[test]
fn error_kind_test() {
    let input = "world";
    let result = literal("hello").parse_next(input);
    
    match result {
        Ok(_) => println!("解析成功"),
        Err(e) => {
            println!("解析失败: {:?}", e);
            if let Some(kind) = e.kind() {
                match kind {
                    ErrorKind::Literal => println!("字面量匹配失败"),
                    _ => println!("其他错误类型: {:?}", kind),
                }
            }
        }
    }
}
```

### 7. multi 模块

#### fold_repeat

**作用**：多次应用解析器，并使用累加函数组合结果。

**示例**：
```rust
use winnow::{multi::fold_repeat, prelude::*, token::literal};

#[test]
fn fold_repeat_test() {
    let input = "abcabcabc";
    let result = fold_repeat(
        0.., // 重复次数
        literal("abc"), // 要重复的解析器
        || 0, // 初始值
        |acc, _| acc + 1 // 累加函数
    ).parse_next(input).unwrap();
    assert_eq!(result, 3);
}
```

#### length_count

**作用**：解析一个长度值，然后解析指定数量的元素。

**示例**：
```rust
use winnow::{multi::length_count, prelude::*, token::{literal, take_while}};

#[test]
fn length_count_test() {
    let input = "3abcabcabc";
    let result = length_count(
        take_while(1.., |c: char| c.is_ascii_digit()).try_map(|s: &str| s.parse::<usize>()),
        literal("abc")
    ).parse_next(input).unwrap();
    assert_eq!(result, vec!["abc", "abc", "abc"]);
}
```

#### length_data

**作用**：解析一个长度值，然后解析指定数量的数据。

**示例**：
```rust
use winnow::{multi::length_data, prelude::*, token::{take_while, take}};

#[test]
fn length_data_test() {
    let input = "3abc";
    let result = length_data(
        take_while(1.., |c: char| c.is_ascii_digit()).try_map(|s: &str| s.parse::<usize>())
    ).parse_next(input).unwrap();
    assert_eq!(result, "abc");
}
```

### 8. sequence 模块

#### preceded

**作用**：解析一个模式，然后解析另一个模式，只返回第二个模式的结果。

**示例**：
```rust
use winnow::{sequence::preceded, prelude::*, token::{literal, take_while}};

#[test]
fn preceded_test() {
    let input = "abc123";
    let result = preceded(
        literal("abc"),
        take_while(1.., |c: char| c.is_ascii_digit())
    ).parse_next(input).unwrap();
    assert_eq!(result, "123");
}
```

#### terminated

**作用**：解析一个模式，然后解析另一个模式，只返回第一个模式的结果。

**示例**：
```rust
use winnow::{sequence::terminated, prelude::*, token::{literal, take_while}};

#[test]
fn terminated_test() {
    let input = "123abc";
    let result = terminated(
        take_while(1.., |c: char| c.is_ascii_digit()),
        literal("abc")
    ).parse_next(input).unwrap();
    assert_eq!(result, "123");
}
```

#### pair

**作用**：解析两个连续的模式，返回它们的结果对。

**示例**：
```rust
use winnow::{sequence::pair, prelude::*, token::{literal, take_while}};

#[test]
fn pair_test() {
    let input = "abc123";
    let result = pair(
        literal("abc"),
        take_while(1.., |c: char| c.is_ascii_digit())
    ).parse_next(input).unwrap();
    assert_eq!(result, ("abc", "123"));
}
```

### 9. branch 模块

#### peek

**作用**：尝试解析模式，但不消耗输入，只返回是否匹配。

**示例**：
```rust
use winnow::{branch::peek, prelude::*, token::literal};

#[test]
fn peek_test() {
    let input = "hello world";
    let result = peek(literal("hello")).parse_next(input).unwrap();
    assert_eq!(result, "hello");
    
    // 输入未被消耗
    assert_eq!(input, "hello world");
}
```

#### not

**作用**：成功当且仅当提供的解析器失败，不消耗输入。

**示例**：
```rust
use winnow::{branch::not, prelude::*, token::literal};

#[test]
fn not_test() {
    let input = "world";
    let result = not(literal("hello")).parse_next(input).unwrap();
    assert_eq!(result, ());
    
    // 输入未被消耗
    assert_eq!(input, "world");
}
```

#### cond

**作用**：根据条件决定是否应用解析器。

**示例**：
```rust
use winnow::{branch::cond, prelude::*, token::literal};

#[test]
fn cond_test() {
    let input = "hello world";
    
    // 条件为true，应用解析器
    let result1 = cond(true, literal("hello")).parse_next(input).unwrap();
    assert_eq!(result1, Some("hello"));
    
    // 条件为false，不应用解析器
    let input2 = "hello world";
    let result2 = cond(false, literal("hello")).parse_next(input2).unwrap();
    assert_eq!(result2, None);
}
```

### 10. bytes 模块

#### take

**作用**：从输入中获取指定数量的字节。

**示例**：
```rust
use winnow::{bytes::take, prelude::*};

#[test]
fn take_test() {
    let input = b"hello world";
    let result = take(5u8).parse_next(input).unwrap();
    assert_eq!(result, b"hello");
}
```

#### take_until

**作用**：从输入中获取字节，直到遇到指定的模式。

**示例**：
```rust
use winnow::{bytes::take_until, prelude::*, token::literal};

#[test]
fn take_until_test() {
    let input = b"hello world";
    let result = take_until(0.., literal(b" ")).parse_next(input).unwrap();
    assert_eq!(result, b"hello");
}
```

#### tag_no_case

**作用**：匹配一个字节序列，不区分大小写。

**示例**：
```rust
use winnow::{bytes::tag_no_case, prelude::*};

#[test]
fn tag_no_case_test() {
    let input = b"HELLO world";
    let result = tag_no_case(b"hello").parse_next(input).unwrap();
    assert_eq!(result, b"HELLO");
}
```

### 11. character 模块

#### one_of

**作用**：匹配输入中的一个字符，该字符必须在提供的字符集中。

**示例**：
```rust
use winnow::{character::one_of, prelude::*};

#[test]
fn one_of_test() {
    let input = "abc";
    let result = one_of(['a', 'b', 'c']).parse_next(input).unwrap();
    assert_eq!(result, 'a');
}
```

#### none_of

**作用**：匹配输入中的一个字符，该字符不能在提供的字符集中。

**示例**：
```rust
use winnow::{character::none_of, prelude::*};

#[test]
fn none_of_test() {
    let input = "xyz";
    let result = none_of(['a', 'b', 'c']).parse_next(input).unwrap();
    assert_eq!(result, 'x');
}
```

#### satisfy

**作用**：匹配输入中满足指定谓词的一个字符。

**示例**：
```rust
use winnow::{character::satisfy, prelude::*};

#[test]
fn satisfy_test() {
    let input = "abc";
    let result = satisfy(|c| c.is_alphabetic()).parse_next(input).unwrap();
    assert_eq!(result, 'a');
}
```

## 实际应用示例

### 解析 JSON 格式的颜色值

```rust
use winnow::{combinator::{delimited, seq}, token::{literal, take_while}, ascii::space0, prelude::*};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn parse_json_color(input: &mut &str) -> PResult<Color> {
    delimited(
        literal("{"),
        seq!(Color {
            _: space0,
            _: literal("\"red\":"),
            red: parse_number,
            _: space0,
            _: literal(","),
            _: space0,
            _: literal("\"green\":"),
            green: parse_number,
            _: space0,
            _: literal(","),
            _: space0,
            _: literal("\"blue\":"),
            blue: parse_number,
            _: space0
        }),
        literal("}")
    ).parse_next(input)
}

fn parse_number(input: &mut &str) -> PResult<u8> {
    take_while(1.., |c: char| c.is_ascii_digit())
        .try_map(|s: &str| s.parse::<u8>())
        .parse_next(input)
}

#[test]
fn json_color_test() {
    let input = "{\"red\": 255, \"green\": 128, \"blue\": 64}";
    let result = parse_json_color.parse_next(input).unwrap();
    assert_eq!(result, Color { red: 255, green: 128, blue: 64 });
}
```

### 解析简单的键值对

```rust
use winnow::{combinator::{delimited, separated_pair}, token::{literal, take_while}, ascii::space0, prelude::*};

fn parse_key_value(input: &mut &str) -> PResult<(String, String)> {
    separated_pair(
        take_while(1.., |c: char| c.is_alphabetic()),
        delimited(space0, literal(":"), space0),
        take_while(1.., |c: char| !c.is_whitespace())
    ).parse_next(input)
}

#[test]
fn key_value_test() {
    let input = "name: John";
    let result = parse_key_value.parse_next(input).unwrap();
    assert_eq!(result, ("name".to_string(), "John".to_string()));
}
```

### 解析简单的配置文件

```rust
use winnow::{
    combinator::{delimited, separated_pair, opt, alt},
    token::{literal, take_while},
    ascii::{space0, space1, digit1, alpha1},
    multi::separated0,
    prelude::*,
};

#[derive(Debug, PartialEq)]
enum ConfigValue {
    String(String),
    Number(i32),
    Boolean(bool),
    Array(Vec<ConfigValue>),
}

#[derive(Debug, PartialEq)]
struct ConfigEntry {
    key: String,
    value: ConfigValue,
}

#[derive(Debug, PartialEq)]
struct Config {
    entries: Vec<ConfigEntry>,
}

// 解析字符串值
fn parse_string_value(input: &mut &str) -> PResult<ConfigValue> {
    take_while(1.., |c: char| !c.is_whitespace() && c != ',' && c != ';')
        .map(|s: &str| ConfigValue::String(s.to_string()))
        .parse_next(input)
}

// 解析数字值
fn parse_number_value(input: &mut &str) -> PResult<ConfigValue> {
    take_while(1.., |c: char| c.is_ascii_digit() || c == '-')
        .try_map(|s: &str| s.parse::<i32>())
        .map(ConfigValue::Number)
        .parse_next(input)
}

// 解析布尔值
fn parse_boolean_value(input: &mut &str) -> PResult<ConfigValue> {
    alt((
        literal("true").map(|_| ConfigValue::Boolean(true)),
        literal("false").map(|_| ConfigValue::Boolean(false)),
    )).parse_next(input)
}

// 解析数组值
fn parse_array_value(input: &mut &str) -> PResult<ConfigValue> {
    delimited(
        literal("["),
        separated0(
            alt((parse_string_value, parse_number_value, parse_boolean_value)),
            delimited(space0, literal(","), space0)
        ),
        literal("]")
    ).map(ConfigValue::Array)
    .parse_next(input)
}

// 解析值
fn parse_value(input: &mut &str) -> PResult<ConfigValue> {
    alt((
        parse_array_value,
        parse_number_value,
        parse_boolean_value,
        parse_string_value,
    )).parse_next(input)
}

// 解析配置条目
fn parse_config_entry(input: &mut &str) -> PResult<ConfigEntry> {
    separated_pair(
        take_while(1.., |c: char| c.is_alphabetic() || c == '_'),
        delimited(space0, literal("="), space0),
        parse_value
    ).map(|(key, value)| ConfigEntry {
        key: key.to_string(),
        value,
    }).parse_next(input)
}

// 解析配置文件
fn parse_config(input: &mut &str) -> PResult<Config> {
    separated0(
        parse_config_entry,
        delimited(space0, literal(";"), space0)
    ).map(|entries| Config { entries })
    .parse_next(input)
}

#[test]
fn config_test() {
    let input = "name = John; age = 30; active = true; tags = [\"dev\", \"rust\"];";
    let result = parse_config(input).unwrap();
    
    let expected = Config {
        entries: vec![
            ConfigEntry {
                key: "name".to_string(),
                value: ConfigValue::String("John".to_string()),
            },
            ConfigEntry {
                key: "age".to_string(),
                value: ConfigValue::Number(30),
            },
            ConfigEntry {
                key: "active".to_string(),
                value: ConfigValue::Boolean(true),
            },
            ConfigEntry {
                key: "tags".to_string(),
                value: ConfigValue::Array(vec![
                    ConfigValue::String("dev".to_string()),
                    ConfigValue::String("rust".to_string()),
                ]),
            },
        ],
    };
    
    assert_eq!(result, expected);
}
```

## 高级用法

### 自定义解析器

```rust
use winnow::{combinator::seq, token::{take_while, literal}, prelude::*};

// 自定义解析器，解析 IPv4 地址
fn parse_ipv4(input: &mut &str) -> PResult<(u8, u8, u8, u8)> {
    seq!(
        _: parse_u8,
        _: literal("."),
        _: parse_u8,
        _: literal("."),
        _: parse_u8,
        _: literal("."),
        _: parse_u8
    ).parse_next(input)
}

fn parse_u8(input: &mut &str) -> PResult<u8> {
    take_while(1..=3, |c: char| c.is_ascii_digit())
        .try_map(|s: &str| s.parse::<u8>())
        .parse_next(input)
}

#[test]
fn ipv4_test() {
    let input = "192.168.1.1";
    let result = parse_ipv4.parse_next(input).unwrap();
    assert_eq!(result, (192, 168, 1, 1));
}
```

### 错误处理和恢复

```rust
use winnow::{combinator::{alt, opt}, token::literal, prelude::*};

// 尝试解析多个可能的选项，提供有意义的错误信息
fn parse_greeting(input: &mut &str) -> PResult<&str> {
    alt((
        literal("hello"),
        literal("hi"),
        literal("hey")
    )).parse_next(input)
}

#[test]
fn greeting_test() {
    let input = "hi there";
    let result = parse_greeting.parse_next(input).unwrap();
    assert_eq!(result, "hi");
}
```

### 自定义错误处理

```rust
use winnow::{error::{ContextError, FromExternalError}, prelude::*, token::literal};

// 自定义错误类型
#[derive(Debug)]
enum ParseError {
    InvalidNumber(String),
    MissingField(String),
    UnknownToken(String),
}

impl<I> ContextError<I> for ParseError {
    fn add_context(_input: I, _ctx: &'static str, mut self: Self) -> Self {
        self
    }
}

impl<I, E: std::fmt::Display> FromExternalError<I, E> for ParseError {
    fn from_external_error(_input: I, _kind: winnow::error::ErrorKind, e: E) -> Self {
        ParseError::InvalidNumber(e.to_string())
    }
}

// 使用自定义错误类型的解析器
fn parse_number(input: &mut &str) -> Result<u32, ParseError> {
    use winnow::token::take_while;
    
    take_while(1.., |c: char| c.is_ascii_digit())
        .try_map(|s: &str| s.parse::<u32>())
        .parse_next(input)
        .map_err(|_| ParseError::InvalidNumber("Invalid number format".to_string()))
}

#[test]
fn custom_error_test() {
    let input = "123";
    let result = parse_number(input).unwrap();
    assert_eq!(result, 123);
    
    let input2 = "abc";
    let result2 = parse_number(input2);
    assert!(matches!(result2, Err(ParseError::InvalidNumber(_))));
}
```

### 性能优化技巧

```rust
use winnow::{combinator::{dispatch, seq}, prelude::*, token::{take_while, literal}};

// 使用 dispatch 进行高效的模式匹配
fn parse_optimized(input: &mut &str) -> PResult<&str> {
    dispatch! {take_while(1.., |c: char| c.is_alphabetic());
        "add" => literal("add").parse_next(input),
        "sub" => literal("sub").parse_next(input),
        "mul" => literal("mul").parse_next(input),
        "div" => literal("div").parse_next(input),
        _ => literal("unknown").parse_next(input),
    }
}

#[test]
fn optimized_test() {
    let input = "add 1 2";
    let result = parse_optimized(input).unwrap();
    assert_eq!(result, "add");
}

// 使用 seq! 宏减少临时分配
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_point(input: &mut &str) -> PResult<Point> {
    seq!(Point {
        x: parse_i32,
        _: literal(","),
        y: parse_i32
    }).parse_next(input)
}

fn parse_i32(input: &mut &str) -> PResult<i32> {
    take_while(1.., |c: char| c.is_ascii_digit() || c == '-')
        .try_map(|s: &str| s.parse::<i32>())
        .parse_next(input)
}

#[test]
fn point_test() {
    let input = "10,20";
    let result = parse_point(input).unwrap();
    assert_eq!(result, Point { x: 10, y: 20 });
}
```

### 实现自定义 Parser trait

```rust
use winnow::{prelude::*, token::take_while};

// 自定义解析器结构体
struct NumberParser;

// 实现 Parser trait
impl<I, E> Parser<I, u32, E> for NumberParser
where
    I: StreamIsPartial,
    I: Stream,
    <I as Stream>::Token: char,
    E: ParseError<I>,
{
    fn parse_next(&mut self, input: &mut I) -> PResult<u32, E> {
        take_while(1.., |c: char| c.is_ascii_digit())
            .try_map(|s: &str| s.parse::<u32>())
            .parse_next(input)
    }
}

#[test]
fn custom_parser_test() {
    let mut parser = NumberParser;
    let input = "123";
    let result = parser.parse_next(input).unwrap();
    assert_eq!(result, 123);
}
```

## 最佳实践

1. **使用 prelude**：导入 `winnow::prelude::*` 可以获取最常用的类型和 trait，简化代码。

2. **组合解析器**：将简单的解析器组合成更复杂的解析器，提高代码的可重用性。

3. **错误处理**：为解析器提供有意义的错误信息，便于调试。

4. **性能考虑**：对于性能敏感的应用，考虑使用 `winnow` 的零成本抽象特性。

5. **测试**：为每个解析器编写测试用例，确保其正确性。

6. **自定义错误类型**：为特定应用场景定义自定义错误类型，提供更精确的错误信息。

7. **使用合适的输入类型**：根据解析的内容选择合适的输入类型，如 `&str` 用于文本，`&[u8]` 用于二进制数据。

8. **避免不必要的分配**：尽量使用切片和引用，避免在解析过程中进行不必要的内存分配。

9. **使用 `dispatch` 进行模式匹配**：对于复杂的分支逻辑，使用 `dispatch` 组合器可以提高代码的可读性和性能。

10. **考虑使用 `Parser` trait**：对于复杂的解析逻辑，实现 `Parser` trait 可以提高代码的组织性和可重用性。

## 总结

Winnow 是一个强大而灵活的解析器组合器库，它提供了丰富的工具来构建各种类型的解析器。通过组合简单的解析器，可以构建出复杂的解析逻辑，适用于解析字符串、二进制数据等各种场景。它的零成本抽象特性使其在性能敏感的应用中也能表现出色。

### Winnow 的主要优势

1. **零成本抽象**：Winnow 的解析器在编译时被优化为高效的代码，没有运行时开销。
2. **组合式设计**：可以将简单的解析器组合成复杂的解析器，提高代码的可重用性和可维护性。
3. **类型安全**：Rust 的类型系统确保解析器的正确性，减少运行时错误。
4. **灵活的输入类型**：支持多种输入类型，包括字符串、字节切片等。
5. **强大的错误处理**：提供详细的错误信息，便于调试和错误恢复。
6. **丰富的组合器**：提供多种组合器，满足不同的解析需求。

### 适用场景

1. **配置文件解析**：解析各种格式的配置文件，如 JSON、TOML、YAML 等。
2. **协议解析**：解析网络协议、二进制协议等。
3. **编程语言解析**：构建编译器或解释器的前端。
4. **数据格式转换**：将一种数据格式转换为另一种数据格式。
5. **文本处理**：处理复杂的文本格式，如日志文件、CSV 文件等。

### 进一步学习的资源

1. **官方文档**：[Winnow 官方文档](https://docs.rs/winnow/latest/winnow/all.html#functions) 提供了详细的 API 参考和示例。
2. **源代码**：阅读 Winnow 的源代码可以帮助理解其内部工作原理。
3. **示例项目**：查看使用 Winnow 的实际项目，学习最佳实践。
4. **社区讨论**：参与 Rust 社区的讨论，获取帮助和分享经验。

希望这篇指南能帮助你理解和使用 Winnow 库。通过实践和探索，你将能够充分利用 Winnow 的强大功能，构建出高效、可靠的解析器。