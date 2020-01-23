use juniper::{EmptyMutation, RootNode};

use crate::types::{Person, Cult};
use crate::models::person::get_person_all;

#[juniper::object(description = "A real human bean")]
impl Person {
  pub fn id(&self) -> i32 {
    self.person_id
  }

  pub fn name(&self) -> &str {
    self.person_name.as_str()
  }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
  fn persons() -> Vec<Person> {
    let mut vec: Vec<Person> = Vec::new();
    get_person_all(&mut vec);
    vec
  }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, EmptyMutation::new())
}
