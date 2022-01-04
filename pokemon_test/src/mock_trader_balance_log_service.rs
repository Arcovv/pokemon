use pokemon_model::dto::InsertTraderBalanceLog;
use pokemon_service::TraderBalanceLogService;

pub struct MockTraderBalanceLogService;

impl TraderBalanceLogService for MockTraderBalanceLogService {
  fn add_log(&self, log: InsertTraderBalanceLog) {
    debug!("trader balance log added: {:?}", log);
  }

  fn add_logs(&self, logs: Vec<InsertTraderBalanceLog>) {
    for log in logs {
      debug!("trader balance log added: {:?}", log);
    }
  }
}
