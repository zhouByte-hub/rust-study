#[cfg(target_os = "macos")]
mod embed_test {
    use rust_embed::Embed;

    #[derive(Embed)]
    #[folder = "src/web/embed_files"]
    struct Asset;

    #[test]
    fn test_1() {
        let data = Asset::get("test.txt").unwrap();
        let content = String::from_utf8(data.data.to_vec()).unwrap();
        assert_eq!("hello rust-embed", content);
    }

    #[test]
    fn test_2() {
        let data = Asset::get("temp/temp.txt").unwrap();
        let content = String::from_utf8(data.data.to_vec()).unwrap();
        println!("{}", content);
    }

    #[test]
    fn test_3() {
        let data = Asset::get("src/web/embed_files/temp/index.html").unwrap();
        let content = String::from_utf8(data.data.to_vec()).unwrap();
        println!("{}", content);

        // 通过响应流将页面数据发送出去，实现将前端嵌套在二进制程序中
        // HttpResponse::Ok().content_type("text/html").body(data.data)
    }
}
