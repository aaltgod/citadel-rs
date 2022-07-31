use actix_web::{get, post, HttpResponse, HttpServer, Responder, App};


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hi there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}