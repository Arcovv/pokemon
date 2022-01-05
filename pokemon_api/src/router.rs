use actix_web::web;

use crate::{matchmaking, trader, user};

pub fn config(config: &mut web::ServiceConfig) {
  config
    .service(matchmaking::post::run_matchmaking)
    .service(user::post::login)
    .service(user::post::register)
    .service(trader::get::fetch_latest_orders_by_card)
    .service(trader::get::fetch_latest_orders_by_trader)
    .service(trader::post::send_buy_order)
    .service(trader::post::send_sell_order);
}
