use actix_web::{post, web};
use serde::{Deserialize, Serialize};
/**
 * Json<T> 允许将请求体反序列化为一个结构体。要从请求体中提取类型信息， T 类型必须实现 serde::Deserialize
 */

#[derive(Debug, Serialize, Deserialize)]

struct User {
    username: String,
    age: i8,
}

#[post("/submit")]
pub async fn submit(user: web::Json<User>) -> String {
    format!("{:?}", user)
}

pub fn json_config_service(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/json").service(submit);
    service_config.service(scope);
}
