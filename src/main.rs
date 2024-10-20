use actix_web::{get, route, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde::Serialize;

use user_admin::sess;
use user_admin::sess_impl::db_mgr as sess_mgr;
use user_admin::user;
use user_admin::user_impl::db_mgr as user_mgr;

#[derive(Debug, Serialize, Deserialize)]
struct Hello {
    message: String,
}

async fn index(item: web::Json<Hello>) -> HttpResponse {
    HttpResponse::Ok().json(item.message.clone()) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").route(web::post().to(index))))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
