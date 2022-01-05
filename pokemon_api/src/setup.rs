use std::sync::RwLock;

use pokemon_infrastructure::repo::{
  DieselCardRepository, DieselOrderRepository, DieselTraderRepository, DieselUserRepository,
};
use pokemon_infrastructure::service::{
  DieselTraderBalanceLogService, ImplMatchmakingService, ImplSendBuyOrderService,
  ImplSendSellOrderService, ImplUserService,
};
use pokemon_infrastructure::{db_helper, DieselOrderQueryCommands, PgConnectionPool};
use pokemon_service::{
  CardRepository, DomainRegistry, MatchmakingService, OrderQueryCommands, OrderRepository,
  SendBuyOrderService, SendSellOrderService, TraderBalanceLogService, TraderRepository,
  UserService, DOMAIN_REGISTRY,
};

pub fn setup() {
  let app_domain_registry = AppDomainRegistry {
    pool: db_helper::CONNECTION_POOL.clone(),
  };

  DOMAIN_REGISTRY
    .set(RwLock::new(Box::new(app_domain_registry)))
    .ok()
    .expect("Failed to set domain registry");
}

struct AppDomainRegistry {
  pool: PgConnectionPool,
}

impl DomainRegistry for AppDomainRegistry {
  fn card_repository(&self) -> Box<dyn CardRepository> {
    let repo = DieselCardRepository::new(self.pool.clone());

    Box::new(repo)
  }

  fn matchmaking_service(&self) -> Box<dyn MatchmakingService> {
    Box::new(ImplMatchmakingService)
  }

  fn order_query_commands(&self) -> Box<dyn OrderQueryCommands> {
    let commands = DieselOrderQueryCommands::new(self.pool.clone());

    Box::new(commands)
  }

  fn order_repository(&self) -> Box<dyn OrderRepository> {
    let repo = DieselOrderRepository::new(self.pool.clone());

    Box::new(repo)
  }

  fn send_buy_order_service(&self) -> Box<dyn SendBuyOrderService> {
    Box::new(ImplSendBuyOrderService)
  }

  fn send_sell_order_service(&self) -> Box<dyn SendSellOrderService> {
    Box::new(ImplSendSellOrderService)
  }

  fn trader_balance_log_service(&self) -> Box<dyn TraderBalanceLogService> {
    let service = DieselTraderBalanceLogService::new(self.pool.clone());

    Box::new(service)
  }

  fn trader_repository(&self) -> Box<dyn TraderRepository> {
    let repo = DieselTraderRepository::new(self.pool.clone());

    Box::new(repo)
  }

  fn user_service(&self) -> Box<dyn UserService> {
    Box::new(ImplUserService)
  }

  fn user_repository(&self) -> Box<dyn pokemon_service::UserRepository> {
    let repo = DieselUserRepository::new(self.pool.clone());

    Box::new(repo)
  }
}
