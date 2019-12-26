extern crate postgres;
use postgres::{Connection, TlsMode};
use std::env;
// static PG_CONNECTION_STRING: &str =  "postgresql://jayy-lmao:yeetus@db:5442";

pub fn get_person_all(vec: &mut Vec<crate::Person>) {
    let pg_connection_string = env::var("DATABASE_URI").expect("need a db uri");
    println!("Connecting to {}", pg_connection_string);
    let conn = Connection::connect(&pg_connection_string[..], TlsMode::None) .unwrap();
    println!("Connection is fine");
    for row in &conn.query("SELECT person_id, person_name FROM persons", &[]).unwrap() {
        let person = crate::Person {
            person_id: row.get(0),
            person_name: row.get(1)
        };
        vec.push(person);
    }
}

pub fn get_person_by_id(vec: &mut Vec<crate::Person>, id: i32) {
    let pg_connection_string = env::var("DATABASE_URI").expect("need a db uri");
    let conn = Connection::connect(&pg_connection_string[..], TlsMode::None) .unwrap();
    for row in &conn.query("SELECT person_id, person_name FROM persons WHERE person_id = $1", &[&id]).unwrap() {
        let person = crate::Person {
            person_id: row.get(0),
            person_name: row.get(1)
        };
        vec.push(person);
    }
}