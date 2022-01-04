use serde::Deserialize;

#[derive(Deserialize)]
pub struct TraderOrderLimitation {
  #[serde(default = "default_limit")]
  pub limit: u32,
}

fn default_limit() -> u32 {
  50
}
