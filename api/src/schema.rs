use juniper::{EmptyMutation, RootNode};

use crate::models::{
  cult::{get_cult_all, get_cult_by_id},
  person::{get_person_all, get_person_by_cult},
};
use crate::types::{Cult, Person};

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
