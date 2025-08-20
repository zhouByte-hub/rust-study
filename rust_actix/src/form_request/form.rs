use actix_web::{post, web};
use serde::{Deserialize, Serialize};

/**
 * web::Form的表单正文可以提取到一个结构体中，类似于 Json<T> 。这种类型必须实现 serde::Deserialize 。
 */

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    age: i8,
}

#[post("/submit_form")]
pub async fn submit_form(form: web::Form<User>) -> String {
    format!("{:?}", form.username)
}

pub fn form_config_service(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/form").service(submit_form);
    service_config.service(scope);
}
