use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::Currency;

#[derive(Debug, Clone, Copy)]
pub struct CardPrice {
  pub cents: i32,
  pub currency: Currency,
}

impl CardPrice {
  pub fn new(cents: i32) -> Self {
    assert!((100 ..= 1000).contains(&cents));

    Self {
      cents,
      currency: Currency::Usd,
    }
  }

  pub fn new2(cents: i32, currency: Currency) -> Self {
    assert!((100 ..= 1000).contains(&cents));

    Self { cents, currency }
  }

  pub fn dollars(self) -> f32 {
    self.cents as f32 / 100.0
  }
}

impl From<i32> for CardPrice {
  fn from(v: i32) -> Self {
    Self {
      cents: v,
      currency: Currency::Usd,
    }
  }
}

impl Add for CardPrice {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    CardPrice {
      cents: self.cents + rhs.cents,
      ..self
    }
  }
}

impl AddAssign for CardPrice {
  fn add_assign(&mut self, rhs: Self) {
    self.cents += rhs.cents
  }
}

impl Sub for CardPrice {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    CardPrice {
      cents: self.cents - rhs.cents,
      ..self
    }
  }
}

impl SubAssign for CardPrice {
  fn sub_assign(&mut self, rhs: Self) {
    self.cents -= rhs.cents
  }
}

impl PartialOrd for CardPrice {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.currency != other.currency {
      return None;
    }

    Some(self.cents.cmp(&other.cents))
  }
}

impl PartialEq for CardPrice {
  fn eq(&self, other: &Self) -> bool {
    if self.currency != other.currency {
      false
    } else {
      self.cents == other.cents
    }
  }
}
