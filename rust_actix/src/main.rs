use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};

/**
 * Actix Web 是一个强大、实用且极快的老牌 Rust Web 框架。
 *
 * 使用 Actix Web 开发的应用程序将暴露一个包含在原生可执行文件中的 HTTP 服务器。你可以将其放在另一个 HTTP 服务器（如 nginx）后面，或者直接提供服务。
 * 即使在完全没有其他 HTTP 服务器的情况下，Actix Web 也足够强大，能够提供 HTTP/1 和 HTTP/2 支持，以及 TLS（HTTPS）。这使得它适用于构建准备投入生产的小型服务。
 */
pub mod controller;
pub mod form_request;
pub mod sse;
use crate::controller::{basic, json};
use crate::sse::sender::SseSender;
use rust_embed::RustEmbed;

/**
 * 应用程序状态: 应用程序状态在相同作用域内的所有路由和资源之间共享。状态可以通过 web::Data<T> 提取器访问
 */
pub struct AppState {
    app_name: String,
}

#[derive(RustEmbed)]
#[folder = "web-front"]
pub struct WebFront;

async fn handle_web_request(req: HttpRequest) -> HttpResponse {
    // 从请求中提取路径参数，是{path:.*}部分，不包含 /front 部分
    let path = req.match_info().query("path").to_string();
    println!("path: {}", path);
    let content_type = mime_guess::from_path(&path).first_or_octet_stream();
    if path == "/" || path.is_empty() {
        let file = WebFront::get("html/login.html");
        return match file {
            Some(body) => HttpResponse::Ok().content_type("text/html").body(body.data),
            None => HttpResponse::Ok().body("404"),
        };
    }
    let file = WebFront::get(&path);
    match file {
        Some(body) => HttpResponse::Ok()
            .content_type(content_type.as_ref())
            .body(body.data),
        None => HttpResponse::Ok().body("404"),
    }
}

#[actix_web::main]
async fn main() {
    /*
     *  1、HttpServer 自动启动一定数量的 HTTP 工作线程，默认情况下该数量等于系统中的物理 CPU 数量。此数量可以通过 HttpServer::workers() 方法覆盖。
     *  2、HttpServer 支持优雅关闭。在接收到停止信号后，工作进程有特定的时间来完成请求服务。超时后仍然存活的工作进程会被强制关闭。默认情况下，关闭超时时间设置为 30 秒。你可以通过 HttpServer::shutdown_timeout() 方法来更改这个参数。
     *  3、Actix Web 保持连接打开，等待后续请求。—— keep_alive
     */
    HttpServer::new(|| {
        App::new()
            .app_data(AppState {
                app_name: String::from("rust_actix"),
            })
            .app_data(web::Data::new(SseSender::new()))
            .configure(basic::basic_path_config)
            .configure(json::json_path_config)
            .configure(form_request::path::path_config_service)
            .configure(form_request::json::json_config_service)
            .configure(form_request::form::form_config_service)
            .configure(form_request::query::query_config_service)
            .route("/sse", web::get().to(sse::sse_endpoint::sse_stream))
            .route("/front/{path:.*}", web::get().to(handle_web_request))
            .route("/sse2", web::get().to(sse_handler))
    })
    .workers(10)
    // .keep_alive(val)
    // .shutdown_timeout(30)
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}

/**
 * Rust与Web结合优先推荐的SSE方式
 * async-stream = "0.3.6"
 */
use actix_web::Responder;
use tokio::time::{Duration, interval};

async fn sse_handler() -> impl Responder {
    let stream = async_stream::stream! {
        let mut interval = interval(Duration::from_secs(1));
        let mut counter = 0_u32;

        loop {
            interval.tick().await;
            counter += 1;
            if counter == 10 {
                break;
            }

            // 构造 SSE 消息
            let message = format!("data: {{\"count\": {}}}\n", counter);

            // ✅ 转换为 Bytes
            yield Ok::<_, actix_web::Error>(web::Bytes::from(message));
        }
    };

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(stream)
}
