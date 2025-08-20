use actix_web::{get, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
}

/**
 * Query<T> 类型提供了提取请求查询参数的功能
 */
#[get("/query")]
pub async fn query(info: web::Query<String>) -> String {
    format!("successful: {}", info)
}

#[get("/query/{username}")]
pub async fn query_user(info: web::Query<User>) -> String {
    format!("successful: {:?}", info)
}

pub fn query_config_service(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/query").service(query).service(query_user);
    service_config.service(scope);
}
