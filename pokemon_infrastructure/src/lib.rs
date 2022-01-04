pub mod db_helper;

mod diesel_card_repo;
mod diesel_order_query_commands;
mod diesel_order_repo;
mod diesel_trader_balance_log_service;
mod diesel_trader_repo;
mod impl_matchmaking_service;
mod impl_send_buy_order_service;
mod impl_send_sell_order_service;

pub use diesel::PgConnection;
pub use diesel_order_query_commands::DieselOrderQueryCommands;

pub mod repo {
  pub use super::diesel_card_repo::DieselCardRepository;
  pub use super::diesel_order_repo::DieselOrderRepository;
  pub use super::diesel_trader_repo::DieselTraderRepository;
}

pub mod service {
  pub use super::diesel_trader_balance_log_service::DieselTraderBalanceLogService;
  pub use super::impl_matchmaking_service::ImplMatchmakingService;
  pub use super::impl_send_buy_order_service::ImplSendBuyOrderService;
  pub use super::impl_send_sell_order_service::ImplSendSellOrderService;
}

pub type PgConnectionManager = diesel::r2d2::ConnectionManager<PgConnection>;
pub type PgConnectionPool = diesel::r2d2::Pool<PgConnectionManager>;
