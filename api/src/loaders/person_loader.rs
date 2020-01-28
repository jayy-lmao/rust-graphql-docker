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
    let mut person_hashmap = HashMap::new();
    person::get_person_by_ids(&mut person_hashmap, keys.to_vec());
    future::ready(keys.iter().map(|key| person_hashmap[key].clone()).collect())
      .unit_error()
      .boxed()
  }
}
