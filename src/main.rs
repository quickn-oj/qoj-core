extern crate actix_web;
extern crate qoj_interface;
extern crate serde_json;
extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate lazy_static;

mod handshake;
mod config;

use actix_web::{App, HttpServer, web};
use std::io;
use config::Config;

lazy_static! {
    static ref CONFIG: Config = Config::new(["./config.yml"].iter().collect()).unwrap();
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096)) 
            .service(handshake::handshake)
    })
    .bind(format!("{}:{}", CONFIG.ip(), CONFIG.port()))?
    .run()
    .await
}
