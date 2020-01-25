use crate::types::{Cult, Person};
use crate::graphql::Context;
use crate::models::{
  cult::get_cult_by_id,
};

// pub struct Person {
//   pub person_id: i32,
//   pub person_name: String,
//   pub cult: Option<i32>,
// }



#[juniper::graphql_object(Context = Context, description = "A real human bean")]
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