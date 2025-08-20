use actix_web::{Error, HttpRequest, Responder, get, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    age: u8,
}

#[get("/{index}")]
pub async fn path_1(index: web::Path<i32>) -> impl Responder {
    format!("path_1: {}", index)
}

#[get("/info/{username}/{age}")]
pub async fn path_2(info: web::Path<(String, i32)>) -> impl Responder {
    format!("path_2: {:?}", info)
}

#[get("/user/{username}/{age}")] //只要名称相同就会映射到User对象中
pub async fn path_3(user: web::Path<User>) -> impl Responder {
    format!("path_3: {:?}", user)
}

#[get("/http/{user_id}")] // 通过请求对象获取参数
pub async fn path_4(req: HttpRequest) -> Result<String, Error> {
    let user_id: u32 = req.match_info().query("user_id").parse().unwrap();
    Ok(format!("successful: {}", user_id))
}

pub fn path_config_service(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/path")
        .service(path_1)
        .service(path_2)
        .service(path_3)
        .service(path_4);
    service_config.service(scope);
}
