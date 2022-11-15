#[macro_use]
extern crate actix_web;

use std::{env, io};
//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use actix_web::{middleware, App, HttpResponse, HttpServer, Responder};


mod recipe;
mod response;
mod ingredient;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(recipe::create)
            .service(recipe::get)
            .service(recipe::delete)
            .service(recipe::list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

