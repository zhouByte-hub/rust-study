/**
 * url = "2.5.4"
 * 
 * Rust 的 URL 库
 */

#[cfg(test)]
mod url_test{
    use url::Url;


    #[test]
    fn test_1(){
        let path = "https://zhangsan:123123@www.baidu.com/_search/page?query=123&name=zhangsan";

        if let Ok(mut url) = Url::parse(path) {
            println!("Scheme: {}", url.scheme());   //  https
            println!("user: {}", url.username());   // zhangsan
            println!("password: {:?}", url.password()); // 123123
            println!("host: {:?}", url.host_str()); // www.baidu.com
            println!("post: {:?}", url.port()); // 80 or None
            println!("path: {}", url.path()); // /_search/page
            println!("query: {}", url.query().unwrap());    // query=123&name=zhangsan

            // 添加参数
            url.query_pairs_mut().append_pair("address", "长沙");

            // 获取参数名称和参数值
            for (key, value) in url.query_pairs() {
                println!("key = {}, value = {}", key, value);
            }
        }else{
            println!("网站不正确");
        }
    }


    #[test]
    fn test_2(){
        let path = Url::parse("https://www.baidu.com");
        if let Ok(mut url) = path {
            url.set_scheme("http").unwrap();
            url.set_host(Some("127.0.0.1")).unwrap();
            url.set_port(Some(80)).unwrap();

            println!("{}", url);
        }
    }
}