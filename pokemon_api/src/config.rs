use envconfig::Envconfig;

#[derive(Clone, Envconfig)]
pub struct Config {
  #[envconfig(from = "HOST")]
  pub host: String,

  #[envconfig(from = "PORT", default = "8666")]
  pub port: u16,

  #[envconfig(from = "CONNECTION_COUNT")]
  pub connection_count: u16,

  #[envconfig(from = "DATABASE_URL")]
  pub database_url: String,

  #[envconfig(from = "JWT_SECRET")]
  pub jwt_secret: String,
}

impl Config {
  pub fn addr(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }
}
