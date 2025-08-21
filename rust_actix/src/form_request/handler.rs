use actix_web::{
    HttpResponse, Responder, Result, get,
    web::{self, Bytes},
};
use futures_util::{StreamExt, stream};
use serde::{Deserialize, Serialize};

/**
 * 请求处理器是一个异步函数，它接受零个或多个参数，这些参数可以从请求中提取（即实现 FromRequest），并返回一个可以转换为 HttpResponse 的类型（即实现 Responder）。
 * 请求处理分为两个阶段。首先调用处理器对象，返回任何实现 Responder 特性的对象。然后，在返回的对象上调用 respond_to() ，将其转换为 HttpResponse 或 Error 。
 *
 * 默认情况下，Actix Web 为一些标准类型提供 Responder 实现，例如 &'static str 、 String 等。
 */

#[get("/responder_string")]
pub async fn responder_string() -> impl Responder {
    "hello world"
}

// 自定义请求处理器，需要实现Responder接口
#[derive(Debug, Serialize, Deserialize)]
pub struct MyResponder {
    pub data: String,
}

impl Responder for MyResponder {
    type Body = actix_web::body::BoxBody;

    // 必须要实现这个方法，customize 方法用于自定义响应，例如设置响应头

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[get("/my_responder")]
pub async fn my_responder() -> MyResponder {
    MyResponder {
        data: "hello world".to_string(),
    }
}

//  接受流式请求体
#[get("/request_stream")]
pub async fn request_stream(mut body: web::Payload) -> impl Responder {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }
    HttpResponse::Ok()
}

// 响应流式数据
#[get("/response_stream")]
pub async fn response_stream() -> Result<HttpResponse> {
    let count = 0;
    let stream = stream::unfold(count, |mut count| async move {
        count += 1;
        if count > 5 {
            None
        } else {
            let msg = format!("data: Hello from server! Count = {}\n\n", count);
            Some((Ok::<Bytes, actix_web::Error>(Bytes::from(msg)), count))
        }
    });

    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .streaming(stream))
}
