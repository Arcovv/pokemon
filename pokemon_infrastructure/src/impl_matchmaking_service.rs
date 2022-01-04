use pokemon_service::errors::CrudError;
use pokemon_service::{MatchmakingService, DOMAIN_REGISTRY};

pub struct ImplMatchmakingService;

impl MatchmakingService for ImplMatchmakingService {
  fn run(&self, card_id: i64) -> Result<(), CrudError> {
    let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();
    let order_repository = domain_registry.order_repository();
    let trader_repository = domain_registry.trader_repository();
    let trader_balance_log_service = domain_registry.trader_balance_log_service();

    let buy_orders = order_repository.fetch_pending_buy_orders_by(card_id)?;

    for order in buy_orders {
      let mut buy_order = order;
      let mut sell_order = if let Some(sell_order_id) = buy_order.sell_order_id {
        order_repository.fetch_by_order_id(sell_order_id)?
      } else {
        continue;
      };

      let buyer_id = buy_order.buyer_id.unwrap();
      let seller_id = sell_order.seller_id.unwrap();
      let mut buyer = trader_repository.fetch_by_id(buyer_id)?;
      let mut seller = trader_repository.fetch_by_id(seller_id)?;

      let buy_price = buy_order.actual_price.unwrap();
      let is_pending_sell_order = sell_order.status.is_pending();
      let is_pending_buy_order = buy_order.status.is_pending();
      let is_trader_enough_balance = buyer.is_enough_balance(buy_price);

      if is_pending_sell_order && is_pending_buy_order && is_trader_enough_balance {
        let buy_log = buyer.balance_decrease(buy_price);
        let sell_log = seller.balance_increase(buy_price);
        trader_balance_log_service.add_logs(vec![buy_log, sell_log]);

        buy_order.mark_as_completed();

        sell_order.actual_price = Some(buy_price);
        sell_order.mark_as_completed();

        order_repository.save_order(sell_order)?;
        trader_repository.save(buyer)?;
        trader_repository.save(seller)?;
      } else {
        buy_order.mark_buy_failed();
      }

      order_repository.save_order(buy_order)?;
    }

    Ok(())
  }
}
