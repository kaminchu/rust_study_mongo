use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(index)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
#[get("/hello")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}