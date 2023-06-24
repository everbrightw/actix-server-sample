mod api;
mod model;
mod repository;

use api::task::{get_task, start_task, submit_task};

use actix_web::{middleware::Logger, web, web::Data, App, HttpServer, Responder};
use repository::ddb::DDBRepository;

// #[actix_web::main]
// aync fn main() -> std::io::Result<()> {
//     std::env::set_var("Rust_LOG", value: "debug");
//     std::env::set_var("RUST_BACKTRACE", "1");
//
//     println!("Hello, world!");
//
//     HttpServer::new( move || {
//         let logger = Logger::default();
//         App::new().wrap(logger)
//
//
//     });
//
// }

async fn hello() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("Rust_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    let config = aws_config::load_from_env().await;
    println!("{:?}", config.region());

    HttpServer::new(move || {
        print!("server created, thread spawned\n");
        // logger
        let logger = Logger::default();

        let ddb_repo: DDBRepository = DDBRepository::init(String::from("task"), config.clone());
        let ddb_data = Data::new(ddb_repo);

        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
            .service(submit_task)
            .service(start_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
