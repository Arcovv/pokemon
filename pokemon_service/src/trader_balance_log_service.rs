use pokemon_model::dto::InsertTraderBalanceLog;

pub trait TraderBalanceLogService {
  fn add_log(&self, log: InsertTraderBalanceLog);

  fn add_logs(&self, logs: Vec<InsertTraderBalanceLog>);
}
