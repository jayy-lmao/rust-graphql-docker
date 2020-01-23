extern crate postgres;
use crate::db::get_db_conn;
use crate::types::{NewPerson, Person};

pub fn get_person_all(vec: &mut Vec<Person>) {
    let conn = get_db_conn();
    for row in &conn
        .query("SELECT person_id, person_name, cult FROM persons", &[])
        .unwrap()
    {
        let person = Person {
            person_id: row.get(0),
            person_name: row.get(1),
            cult: row.get(2),
        };
        vec.push(person);
    }
}

pub fn get_person_by_ids(vec: &mut Vec<Person>, ids: Vec<i32>) {
    let conn = get_db_conn();
    for row in &conn
        .query(
            "SELECT person_id, person_name, cult FROM persons WHERE person_id = ANY($1)",
            &[&ids],
        )
        .unwrap()
    {
        let person = Person {
            person_id: row.get(0),
            person_name: row.get(1),
            cult: row.get(2),
        };
        vec.push(person);
    }
}

pub fn get_person_by_id(vec: &mut Vec<Person>, id: i32) {
    let conn = get_db_conn();
    for row in &conn
        .query(
            "select person_id, person_name, cult from persons where person_id = $1",
            &[&id],
        )
        .unwrap()
    {
        let person = Person {
            person_id: row.get(0),
            person_name: row.get(1),
            cult: row.get(2),
        };
        vec.push(person);
    }
}

pub fn get_person_by_cult(vec: &mut Vec<Person>, cult: i32) {
    let conn = get_db_conn();
    let res = &conn
        .query(
            "select person_id, person_name, cult from persons where cult = $1",
            &[&cult],
        )
        .unwrap();
    for row in res {
        let person = Person {
            person_id: row.get(0),
            person_name: row.get(1),
            cult: row.get(2),
        };
        vec.push(person);
    }
}

pub fn create_person(data: NewPerson) -> Person {
    let conn = get_db_conn();
    let res = &conn
        .query(
            "INSERT INTO persons (person_name, cult) VALUES ($1, $2) RETURNING person_id, person_name, cult;",
            &[&data.name, &data.cult],
        )
        .unwrap();
    let row = res.iter().next().unwrap();
    Person {
        person_id: row.get(0),
        person_name: row.get(1),
        cult: row.get(2)
    }
}
