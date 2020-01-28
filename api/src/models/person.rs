extern crate postgres;
use crate::db::get_db_conn;
use crate::graphql::model::Person;
use std::collections::HashMap;

// pub fn get_person_all(vec: &mut Vec<Person>) {
//     let conn = get_db_conn();
//     for row in &conn
//         .query("SELECT id, name, cult FROM persons", &[])
//         .unwrap()
//     {
//         let person = Person {
//             id: row.get(0),
//             name: row.get(1),
//             cult: row.get(2),
//         };
//         vec.push(person);
//     }
// }

pub fn get_person_by_ids(hashmap: &mut HashMap<i32, Person>, ids: Vec<i32>) {
    let conn = get_db_conn();
    for row in &conn
        .query(
            "SELECT id, name, cult FROM persons WHERE id = ANY($1)",
            &[&ids],
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

// pub fn get_person_by_id(vec: &mut Vec<Person>, id: i32) {
//     let conn = get_db_conn();
//     for row in &conn
//         .query(
//             "select id, name, cult from persons where id = $1",
//             &[&id],
//         )
//         .unwrap()
//     {
//         let person = Person {
//             id: row.get(0),
//             name: row.get(1),
//             cult: row.get(2),
//         };
//         vec.push(person);
//     }
// }

// pub fn get_person_by_cult(vec: &mut Vec<Person>, cult: i32) {
//     let conn = get_db_conn();
//     let res = &conn
//         .query(
//             "select id, name, cult from persons where cult = $1",
//             &[&cult],
//         )
//         .unwrap();
//     for row in res {
//         let person = Person {
//             id: row.get(0),
//             name: row.get(1),
//             cult: row.get(2),
//         };
//         vec.push(person);
//     }
// }

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
