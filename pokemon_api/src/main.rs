#[macro_use]
extern crate log;

mod api_error;
mod config;
mod matchmaking;
mod prelude;
mod router;
mod setup;
mod token;
mod trader;
mod user;

use std::io;

use actix_web::{middleware, web, App, HttpServer};
use config::Config;
use envconfig::Envconfig;

#[actix_web::main]
async fn main() -> io::Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  setup::setup();

  let config = Config::init_from_env().expect("Failed to load config");
  let addr = config.addr();

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(config.clone()))
      .configure(router::config)
      .wrap(middleware::Logger::default())
  })
  .workers(4)
  .bind(&addr)
  .unwrap()
  .run()
  .await
}
