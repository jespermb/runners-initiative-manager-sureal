use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Combatten {
  pub id: Option<String>,
  pub name: String,
  pub initiative: u32,
  pub health: u32,
  pub damage: u32
}