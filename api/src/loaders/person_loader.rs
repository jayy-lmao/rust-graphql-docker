use crate::models::person;
use crate::graphql::model::Person;
use dataloader::{BatchFn, BatchFuture};
use futures::{future, FutureExt as _ };
use std::collections::HashMap;

pub struct PersonBatcher;

impl BatchFn<i32, Person> for PersonBatcher {
  type Error = ();

  fn load(&self, keys: &[i32]) -> BatchFuture<Person, Self::Error> {
    println!("load batch {:?}", keys);
    let mut vec: Vec<Person> = Vec::new();

    // Get person vector
    person::get_person_by_ids(&mut vec, keys.to_vec());

    // Create a HashMap for getting a person for a given id
    let mut person_hashmap = HashMap::new();
    for (index, person) in vec.iter().enumerate() {
      person_hashmap.insert(person.person_id, index);
    }
    let mut result = Vec::new();
    for key in keys {
      let index = person_hashmap[key];
      result.push(vec[index].clone());
    }

    // Map the keys to the people they are trying to get.
    future::ready(result)
      .unit_error()
      .boxed()
  }
}
