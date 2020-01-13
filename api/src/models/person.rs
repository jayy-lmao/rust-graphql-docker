extern crate postgres;
use crate::db::get_db_conn;
use crate::types::Person;

pub fn get_person_all(vec: &mut Vec<Person>) {
    let conn = get_db_conn();
    for row in &conn
        .query("SELECT person_id, person_name FROM persons", &[])
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
    //TODO: In desperate need of refactor, or else ye shall face the n+1 problem!!

    for id in ids.iter() {
        get_person_by_id(vec, *id);
    };
}

pub fn get_person_by_id(vec: &mut Vec<Person>, id: i32) {
    // let pg_connection_string = env::var("DATABASE_URI").expect("need a db uri");
    // let conn = Connection::connect(&pg_connection_string[..], TlsMode::None).unwrap();
    let conn = get_db_conn();
    for row in &conn
        .query(
            "SELECT person_id, person_name, cult FROM persons WHERE person_id = $1",
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
