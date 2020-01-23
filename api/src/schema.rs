use juniper::{EmptyMutation, RootNode};

use crate::models::{
  cult::get_cult_all,
  person::get_person_all,
};
use crate::types::{Cult, Person};
pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
  fn persons() -> Vec<Person> {
    let mut vec: Vec<Person> = Vec::new();
    get_person_all(&mut vec);
    vec
  }

  fn cults() -> Vec<Cult> {
    let mut vec: Vec<Cult> = Vec::new();
    get_cult_all(&mut vec);
    vec
  }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, EmptyMutation::new())
}
