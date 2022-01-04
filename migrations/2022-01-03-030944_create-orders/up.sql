CREATE TABLE orders (
  "id" bigint GENERATED always AS IDENTITY PRIMARY KEY,
  "kind" text NOT NULL,
  "status" text NOT NULL,
  "buyer_id" bigint REFERENCES traders(id),
  "seller_id" bigint REFERENCES traders(id),
  "buy_order_id" bigint REFERENCES orders(id),
  "sell_order_id" bigint REFERENCES orders(id),
  "card_id" bigint NOT NULL REFERENCES cards(id),
  "expected_price" int NOT NULL DEFAULT 0,
  "actual_price" int,
  "created_at" timestamp WITH time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);