/**
 * mime_guess = "2.0.5"
 * 
 * mime_guess 是一个 Rust 语言的第三方库（crate）。它的主要功能是根据文件的扩展名或内容来猜测文件的 MIME 类型。
 * 
 * MIME 类型 (Multipurpose Internet Mail Extensions)：是一种标准，用来标识文件的性质和格式。例如：
 *      1、text/html 表示 HTML 文件
 *      2、image/jpeg 表示 JPEG 图片
 *      3、application/json 表示 JSON 文件
 *      4、text/plain 表示纯文本文件
 */

#[cfg(test)]
mod mime_test {

    #[test]
    fn test_1() {
        // 根据文件扩展名猜测 MIME 类型
        let mime = mime_guess::from_path("index.html");
        println!("count = {}", mime.count());   // 猜测的结果，一般只有一个，但是可以选择多个
        println!("first = {}", mime.first().unwrap());  // text/html
        println!("first_or_octet_stream = {}", mime.first_or_octet_stream());   // text/html
        println!("first_or_text_plain = {}", mime.first_or_text_plain());       // text/html
        println!("is_empty = {}", mime.is_empty());     // false
        mime.iter().for_each(|item| {
            println!("{}", item);   // text/html
        });
    }

    #[test]
    fn test_2() {
        // 根据扩展名字符串猜测
        let mime = mime_guess::from_ext("html");
        println!("count = {}", mime.count());   // 猜测的结果，一般只有一个，但是可以选择多个
        println!("first = {}", mime.first().unwrap());  // text/html
        println!("first_or_octet_stream = {}", mime.first_or_octet_stream());   // text/html
        println!("first_or_text_plain = {}", mime.first_or_text_plain());       // text/html
        println!("is_empty = {}", mime.is_empty());     // false
        mime.iter().for_each(|item| {
            println!("{}", item);   // text/html
        });
    }
}