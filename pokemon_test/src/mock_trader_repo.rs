use chrono::Utc;
use pokemon_model::Trader;
use pokemon_service::errors::CrudError;
use pokemon_service::TraderRepository;

pub struct MockTraderRepository;

impl TraderRepository for MockTraderRepository {
  #[rustfmt::skip]
  fn fetch_all_traders(&self) -> Result<Vec<Trader>, CrudError> {
    let traders = vec![
      Trader { id: 1, user_id: 1, balance: 10_000, created_at: Utc::now() },
      Trader { id: 2, user_id: 2, balance: 20_000, created_at: Utc::now() },
      Trader { id: 3, user_id: 3, balance: 30_000, created_at: Utc::now() },
    ];

    Ok(traders)
  }

  fn fetch_by_id(&self, _trader_id: i64) -> Result<Trader, CrudError> {
    let trader = Trader {
      id: 1,
      user_id: 1,
      balance: 10_000,
      created_at: Utc::now(),
    };

    Ok(trader)
  }

  fn save(&self, _trader: Trader) -> Result<(), CrudError> {
    Ok(())
  }
}
