use chrono::Utc;
use pokemon_model::Card;
use pokemon_service::errors::CrudError;
use pokemon_service::CardRepository;

pub struct MockCardRepository;

impl CardRepository for MockCardRepository {
  #[rustfmt::skip]
  fn fetch_all_cards(&self) -> Result<Vec<Card>, CrudError> {
    let cards = vec![
      Card { id: 1, name: "Pikachu".into(), created_at: Utc::now() },
      Card { id: 2, name: "Bulbasaur".into(), created_at: Utc::now() },
      Card { id: 3, name: "Charmander".into(), created_at: Utc::now() },
      Card { id: 4, name: "Squirtle".into(), created_at: Utc::now() },
    ];

    Ok(cards)
  }

  fn fetch_by_id(&self, _card_id: i64) -> Result<Card, CrudError> {
    let card = Card {
      id: 1,
      name: "Pikachu".into(),
      created_at: Utc::now(),
    };

    Ok(card)
  }
}
