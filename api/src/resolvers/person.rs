use crate::types::{Cult, Person};
use crate::models::{
  cult::get_cult_by_id,
};

#[juniper::object(description = "A real human bean")]
impl Person {
  pub fn id(&self) -> i32 {
    self.person_id
  }

  pub fn name(&self) -> &str {
    self.person_name.as_str()
  }

  pub fn cult(&self) -> Vec<Cult> {
    let mut vec: Vec<Cult> = Vec::new();
    match self.cult {
      Some(cult_id) => get_cult_by_id(&mut vec, cult_id),
      None => (),
    }
    vec
  }
}