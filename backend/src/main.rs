use actix_cors::Cors;

use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, HttpServer,
    Responder, App,
};
use types::DirResponse;


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hi there")
}

#[get("/root")]
async fn get_root() -> Json<DirResponse> {
    println!("{}", 1);
    Json(DirResponse {
        name: "root".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let allowed_origin: String = "http://127.0.0.1:8080".to_string();
    

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(allowed_origin.as_str())
            .allowed_methods(vec!["GET"]);
        
        App::new().wrap(cors)
            .service(home)
            .service(get_root)
    })
    .bind(("localhost", 8082))?
    .run()
    .await
}