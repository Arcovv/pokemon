use itertools::Itertools;
use pokemon_model::{CardPrice, Order};
use pokemon_service::errors::CrudError;
use pokemon_service::{SendSellOrderService, DOMAIN_REGISTRY};

pub struct ImplSendSellOrderService;

impl SendSellOrderService for ImplSendSellOrderService {
  fn send(
    &self,
    card_id: i64,
    trader_id: i64,
    expected_price: CardPrice,
  ) -> Result<bool, CrudError> {
    let domain_registry = DOMAIN_REGISTRY.get().unwrap().read().unwrap();
    let order_repository = domain_registry.order_repository();

    let buy_orders = order_repository.fetch_pending_buy_orders_by(card_id)?;

    let mut sell_order = Order::create_sell_order(trader_id, card_id, expected_price);

    if buy_orders.is_empty() {
      order_repository.save_order(sell_order)?;
      return Ok(false);
    }

    let highest_order = buy_orders
      .iter()
      .sorted_by(|a, b| b.expected_price.partial_cmp(&a.expected_price).unwrap())
      .next()
      .unwrap();

    let highest_price = highest_order.expected_price;

    let result = if sell_order.expected_price >= highest_price {
      sell_order.buy_order_id = Some(highest_order.id);
      sell_order.actual_price = Some(highest_price);
      true
    } else {
      false
    };

    order_repository.save_order(sell_order)?;
    Ok(result)
  }
}
