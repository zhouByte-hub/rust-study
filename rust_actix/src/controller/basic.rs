use crate::AppState;
use actix_web::{HttpResponse, Responder, get, post, web};

#[get("/hello_world")]
pub async fn helloworld() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

#[get("/web_1/{index}")]
pub async fn web_1(data: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("web_1: {}", data))
}

#[post("/web_2")]
pub async fn web_2(body: web::Bytes) -> impl Responder {
    HttpResponse::Ok().body(format!("web_2: {:?}", body))
}

#[get("/app_name")]
async fn app_name(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

pub fn basic_path_config(service_config: &mut web::ServiceConfig) {
    let user_scope = web::scope("/user")
        .service(helloworld)
        .service(web_1)
        .service(web_2)
        .service(app_name);
    service_config.service(user_scope);
}
