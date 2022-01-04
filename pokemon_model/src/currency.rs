#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, ToString, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Currency {
  Usd,
}
