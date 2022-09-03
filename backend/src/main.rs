use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, HttpServer,
    Responder, App,
};
use actix_web_lab::web::spa;

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
    let allowed_origin: String = "http://localhost:8082".to_string();
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(allowed_origin.as_str());
        
        App::new()
            .wrap(cors)
            .service(get_root)
            .service(
                spa()
                .index_file("./static/index.html")
                .static_resources_mount("/")
                .static_resources_location("./static")
                .finish()
            )
    })
    .bind(("localhost", 8082))?
    .run()
    .await
}