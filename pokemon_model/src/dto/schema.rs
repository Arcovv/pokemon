table! {
    cards (id) {
        id -> Int8,
        name -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    orders (id) {
        id -> Int8,
        kind -> Text,
        status -> Text,
        buyer_id -> Nullable<Int8>,
        seller_id -> Nullable<Int8>,
        buy_order_id -> Nullable<Int8>,
        sell_order_id -> Nullable<Int8>,
        card_id -> Int8,
        expected_price -> Int4,
        actual_price -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

table! {
    traders (id) {
        id -> Int8,
        user_id -> Int8,
        balance -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    traders_balance_logs (id) {
        id -> Int8,
        previous_value -> Int4,
        current_value -> Int4,
        modify_value -> Int4,
        reason -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int8,
        nickname -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(orders -> cards (card_id));
joinable!(traders -> users (user_id));

allow_tables_to_appear_in_same_query!(cards, orders, traders, traders_balance_logs, users,);
