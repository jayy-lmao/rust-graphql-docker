use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
  pub person_id: i32,
  pub person_name: String,
  pub cult: Option<i32>,
}

#[derive(Serialize)]
pub struct Cult {
  pub id: i32,
  pub name: String,
}
