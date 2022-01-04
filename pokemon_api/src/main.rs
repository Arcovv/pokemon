#[macro_use]
extern crate log;

mod api_error;
mod matchmaking;
mod router;
mod setup;
mod trader;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  setup::setup();

  let host = env::var("HOST").expect("Missing HOST env");

  HttpServer::new(move || {
    App::new()
      .configure(router::config)
      .wrap(middleware::Logger::default())
  })
  .workers(4)
  .bind(&format!("{}:8666", host))
  .unwrap()
  .run()
  .await
}
