extern crate postgres;
use crate::db::get_db_conn;
use crate::graphql::resolvers::Person;
use dataloader::Loader;
use dataloader::{BatchFn, BatchFuture};
use futures::{future, FutureExt as _};
use std::collections::HashMap;

pub fn get_persons_by_cult_id(hashmap: &mut HashMap<i32, Person>, cult_id: Vec<i32>) {
    let conn = get_db_conn();
    for row in &conn
        .query(
            "SELECT id, name, cult FROM persons WHERE cult = ANY($1)",
            &[&cult_id],
        )
        .unwrap()
    {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            cult: row.get(2),
        };
        hashmap.insert(person.id, person);
    }
}

// pub fn create_person(data: NewPerson) -> Person {
//     let conn = get_db_conn();
//     let res = &conn
//         .query(
//             "INSERT INTO persons (name, cult) VALUES ($1, $2) RETURNING id, name, cult;",
//             &[&data.name, &data.cult],
//         )
//         .unwrap();
//     let row = res.iter().next().unwrap();
//     Person {
//         id: row.get(0),
//         name: row.get(1),
//         cult: row.get(2)
//     }
// }


pub struct PersonBatcher;

impl BatchFn<i32, Person> for PersonBatcher {
  type Error = ();

  fn load(&self, keys: &[i32]) -> BatchFuture<Person, Self::Error> {
    println!("load batch {:?}", keys);
    let mut person_hashmap = HashMap::new();
    get_persons_by_cult_id(&mut person_hashmap, keys.to_vec());
    future::ready(keys.iter().map(|key| person_hashmap[key].clone()).collect())
      .unit_error()
      .boxed()
  }
}

pub type PersonLoader = Loader<i32, Person, (), PersonBatcher>;

pub fn get_loader() -> PersonLoader {
  Loader::new(PersonBatcher)
}
