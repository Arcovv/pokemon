use itertools::Itertools;
use pokemon_model::{CardPrice, Order};
use pokemon_service::errors::CrudError;
use pokemon_service::{SendBuyOrderService, DOMAIN_REGISTRY};

pub struct ImplSendBuyOrderService;

impl SendBuyOrderService for ImplSendBuyOrderService {
  fn send(
    &self,
    card_id: i64,
    trader_id: i64,
    expected_price: CardPrice,
  ) -> Result<bool, CrudError> {
    let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();
    let order_repository = domain_registry.order_repository();

    let sell_orders = order_repository.fetch_pending_sell_orders_by(card_id)?;

    let mut buy_order = Order::create_buy_order(trader_id, card_id, expected_price);

    // No cards can be sold
    if sell_orders.is_empty() {
      order_repository.save_order(buy_order)?;
      return Ok(false);
    }

    let lowest_order = sell_orders
      .iter()
      .sorted_by(|a, b| a.expected_price.partial_cmp(&b.expected_price).unwrap())
      .next()
      .unwrap();

    let lowest_price = lowest_order.expected_price;

    let result = if buy_order.expected_price >= lowest_price {
      buy_order.sell_order_id = Some(lowest_order.id);
      buy_order.actual_price = Some(lowest_price);
      true
    } else {
      false
    };

    order_repository.save_order(buy_order)?;
    Ok(result)
  }
}
