extern crate postgres;
use crate::db::get_db_conn;
use crate::type_defs::Person;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use async_trait::async_trait;
use std::collections::HashMap;

pub fn get_persons_by_cult_ids(person_vec: &mut Vec<Person>, cult_ids: Vec<i32>) {
    let conn = get_db_conn();
    for row in &conn
        .query(
            "SELECT id, name, cult FROM persons WHERE cult = ANY($1)",
            &[&cult_ids],
        )
        .unwrap()
    {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            cult: row.get(2),
        };
        person_vec.push(person);
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

fn copy_by_cult_id(vec: &Vec<Person>, id: i32) -> Vec<Person> {
    let mut res = Vec::new();
    for p in vec {
        if p.cult == Some(id) {
            res.push(p.clone());
        }
    }
    res
}

pub struct PersonsBatcher;

#[async_trait]
impl BatchFn<i32, Vec<Person>> for PersonsBatcher {

    async fn load(&self, keys: &[i32]) -> HashMap<i32, Vec<Person>> {
        println!("load persons batch {:?}", keys);
        let mut person_vec = Vec::new();
        get_persons_by_cult_ids(&mut person_vec, keys.to_vec());
        keys.iter()
            .map(|&key| (key.clone(), copy_by_cult_id(&person_vec, key)))
            .collect()
    }
}

pub type PersonsLoader = Loader<i32, Vec<Person>, PersonsBatcher>;

pub fn get_loader() -> PersonsLoader {
    Loader::new(PersonsBatcher)
        .with_yield_count(100)
}
