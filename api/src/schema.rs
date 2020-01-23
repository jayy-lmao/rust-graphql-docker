use juniper::{RootNode, FieldResult};

use crate::models::{
  cult::{
    get_cult_all,
    create_cult
  },
  person::{
    get_person_all,
    create_person
  },
};
use crate::types::{
  Cult, 
  NewCult,
  Person, 
  NewPerson
};

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

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
   fn create_person(data: NewPerson) -> Person{
      create_person(data)
   }
   fn create_cult(data: NewCult) -> FieldResult<Option<Cult>> {
      match create_cult(data) {
        Ok(cult) => Ok(cult),
        Err(e) => Err(e)?,
      }
   }
}


pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
