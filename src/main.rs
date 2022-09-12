mod presentation;
mod repository;


use presentation::language::{
    get_language,
};


use actix_web::{HttpServer,App,web::Data, middleware::Logger};
use repository::ddb::DDBRepository;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;

HttpServer::new(move || {
    let ddb_repository: DDBRepository = DDBRepository::init(
        String::from("language"),
        config.clone(),
    );
    let ddb_data = Data::new(ddb_repository);
    let logger = Logger::default();
    App::new()
    .wrap(logger)
    .app_data(ddb_data)
    .service(get_language)
})
.bind(("127.0.0.1", 80))?
.run()
.await

}
