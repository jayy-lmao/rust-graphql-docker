use crate::models::person;
use crate::types::Person;
use dataloader::{BatchFn, BatchFuture, Loader};
use futures::{executor, future, FutureExt as _, TryFutureExt as _};
use std::collections::HashMap;

pub struct Batcher;

impl BatchFn<i32, Person> for Batcher {
  type Error = ();

  fn load(&self, keys: &[i32]) -> BatchFuture<Person, Self::Error> {
    println!("load batch {:?}", keys);
    let vec: Vec<Person> = Vec::new();

    // Get person vector
    person::get_person_by_ids(&mut vec, keys.to_vec());

    // Create a HashMap for getting a person for a given id
    let person_hashmap = HashMap::new();
    for person in vec {
      person_hashmap.insert(person.person_id, person);
    }

    // Map the keys to the people they are trying to get.
    future::ready(keys.into_iter().map(|pid| *person_hashmap.get(pid).unwrap()).collect())
      .unit_error()
      .boxed()
  }
}

pub fn person_loader() -> Loader<i32, Person, (), Batcher> {
  let loader = Loader::new(Batcher);
  loader
}
