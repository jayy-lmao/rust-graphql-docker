use crate::types::{Cult, Person};
use crate::models::person:: get_person_by_cult;

#[juniper::object(description = "A real human CULT")]
impl Cult {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn members(&self) -> Vec<Person> {
    let mut vec: Vec<Person> = Vec::new();
    get_person_by_cult(&mut vec, self.id);
    vec
  }
}