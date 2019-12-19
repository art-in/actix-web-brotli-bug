use std::io;

use actix_files as fs;
use actix_web::{middleware, App, HttpServer};

static BIND_URL: &str = "0.0.0.0:88";

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    println!("Listening on {}", BIND_URL);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new(r#""%r" - %s - %Ts"#))
            .wrap(middleware::Compress::default())
            .service(fs::Files::new("/", "static").index_file("index.html"))
    })
    .bind(BIND_URL)?
    .start()
    .await
}
