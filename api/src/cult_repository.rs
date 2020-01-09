extern crate postgres;
use postgres::{Connection, TlsMode};
use std::env;

use crate::types::{Cult};

pub fn get_cult_all(vec: &mut Vec<Cult>) {
    let pg_connection_string = env::var("DATABASE_URI").expect("need a db uri");
    println!("Connecting to {}", pg_connection_string);
    let conn = Connection::connect(&pg_connection_string[..], TlsMode::None) .unwrap();
    println!("Connection is fine");
    for row in &conn.query("SELECT id, name FROM cults", &[]).unwrap() {
        let cult = Cult {
            id: row.get(0),
            name: row.get(1)
        };
        vec.push(cult);
    }
}

pub fn get_cult_by_id(vec: &mut Vec<Cult>, id: i32) {
    let pg_connection_string = env::var("DATABASE_URI").expect("need a db uri");
    let conn = Connection::connect(&pg_connection_string[..], TlsMode::None) .unwrap();
    for row in &conn.query("SELECT id, name FROM cults WHERE id = $1", &[&id]).unwrap() {
        let cult = Cult {
            id: row.get(0),
            name: row.get(1)
        };
        vec.push(cult);
    }
}
