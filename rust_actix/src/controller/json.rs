use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct User {
    username: String,
    age: i32,
}

#[post("/web_3")]
pub async fn web_3(body: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("web_3: {:?}", body))
}

pub fn json_path_config(service_config: &mut web::ServiceConfig) {
    let scope = web::scope("/json").service(web_3);
    service_config.service(scope);
}
