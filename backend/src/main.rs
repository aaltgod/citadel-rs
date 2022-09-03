use std::{fs, env::set_var};

use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, HttpServer,
    Responder, App,
    middleware::Logger,
};
use actix_web_lab::web::spa;

use common::{
    config::{Config, CONFIG_FILENAME},
    DirResponse,
};


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
    let config_string = fs::read_to_string("../".to_string()+CONFIG_FILENAME)?;
    let config: Config = toml::from_str(&config_string)?;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("INFO"));
    
    
    HttpServer::new(move || {    
        App::new()
            .wrap(Logger::default().log_target("http_log"))
            .service(get_root)
            .service(
                spa()
                .index_file("./static/index.html")
                .static_resources_mount("/")
                .static_resources_location("./static")
                .finish()
            )
    })
    .bind((config.server.address, config.server.port))?
    .run()
    .await
}