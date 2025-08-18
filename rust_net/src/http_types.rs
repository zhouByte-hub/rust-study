/**
 * http-types = "2.12.0"
 * 
 * http-types 是一个专注于提供 HTTP 协议核心数据类型 的 Rust 库，主要作用有：
 *  1、提供标准类型：它定义了处理 HTTP 消息（请求和响应）所需的核心结构体和枚举:
 *      1、Request: 表示一个 HTTP 请求（包含方法、URL、头、体等）
 *      2、Response: 表示一个 HTTP 响应（包含状态码、头、体等）
 *      3、Url: 用于解析、构建和操作 URL
 *      4、Method: 表示 HTTP 方法（如 GET, POST, PUT, DELETE 等）
 *      5、StatusCode: 表示 HTTP 状态码（如 200 OK, 404 Not Found, 500 Internal Server Error 等）
 *      6、Headers: 一个键值对集合，用于存储 HTTP 头部字段。
 *      7、Body: 用于封装请求或响应的主体内容（支持字符串、字节、流等）
 *  2、解耦网络 I/O：http-types 本身 不负责 网络通信（如发送请求、监听端口）。它只关注于定义这些消息的结构。
 *      这使得它可以被不同的 HTTP 客户端或服务器实现所复用，实现了关注点分离。
 *  3、作为基础构件：许多其他 Rust Web 库（尤其是较早的或特定生态的）会使用 http-types 作为它们内部表示请求和响应的基础类型。
 *      例如，tide (一个轻量级 Web 框架) 和 async-h1 (一个 HTTP/1.x 客户端/服务器库) 就构建在 http-types 之上。
 * 
 *  总结来说：http-types 就像是一个“工具箱”，提供了构建 HTTP 消息所需的“螺丝、钉子、木板”（即各种类型），但不提供“锤子和锯子”（即网络传输功能）。
 *  你需要用这些“材料”配合其他库（如 async-h1, smol, async-tls 等）来完成实际的网络操作。
 */

#[cfg(test)]
mod http_test{
    use http_types::{Request, Response, StatusCode, Url};

    #[test]
    fn create_request(){
        let url = Url::parse("http://127.0.0.1:9428/select/logsql/query?query=*&limit=1").unwrap();
        let mut request =  Request::new(http_types::Method::Get, url);

        // 设置请求头
        request.insert_header("Content-Type", "application/json");

        // 设置请求体
        request.set_body("requestBody");
    }

    #[test]
    fn create_response(){
        let mut response = Response::new(StatusCode::Ok);

        // 设置响应头
        response.insert_header("Cookie", "juiu1ljsnfu8i12398afdsj-asjdfhi8181");
        response.insert_header("Content-Type", "application/json");

        // 设置响应体
        response.set_body("this is responseBody");
    }


    #[test]
    fn parse_url(){
        let mut url = Url::parse("http://localhost:9428/select/logsql/query?query=*&limi=1").unwrap();

        println!("Scheme = {}", url.scheme());
        println!("Host = {:?}", url.host());
        println!("username = {}", url.username());
        println!("password = {:?}", url.password());
        println!("Port = {:?}", url.port());
        println!("query = {:?}", url.query());
        println!("path = {}", url.path());

        url.set_host(Some("127.0.0.1")).unwrap();
        url.query_pairs_mut().append_pair("start", "2025-08-18T22:49:00");

        println!("{}", url);
    }

}