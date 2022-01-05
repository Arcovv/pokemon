use std::sync::RwLock;

use once_cell::sync::OnceCell;

use crate::{
  CardRepository, MatchmakingService, OrderQueryCommands, OrderRepository, SendBuyOrderService,
  SendSellOrderService, TraderBalanceLogService, TraderRepository, UserRepository, UserService,
};

pub static DOMAIN_REGISTRY: OnceCell<RwLock<Box<dyn DomainRegistry>>> = OnceCell::new();

pub trait DomainRegistry: Sync + Send {
  fn card_repository(&self) -> Box<dyn CardRepository>;
  fn matchmaking_service(&self) -> Box<dyn MatchmakingService>;
  fn order_query_commands(&self) -> Box<dyn OrderQueryCommands>;
  fn order_repository(&self) -> Box<dyn OrderRepository>;
  fn send_buy_order_service(&self) -> Box<dyn SendBuyOrderService>;
  fn send_sell_order_service(&self) -> Box<dyn SendSellOrderService>;
  fn trader_balance_log_service(&self) -> Box<dyn TraderBalanceLogService>;
  fn trader_repository(&self) -> Box<dyn TraderRepository>;
  fn user_repository(&self) -> Box<dyn UserRepository>;
  fn user_service(&self) -> Box<dyn UserService>;
}
