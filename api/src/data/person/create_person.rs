
use crate::type_defs::{Person,NewPerson};
use crate::db::get_db_conn;

pub fn create_person(data: NewPerson) -> Person {
    let conn = get_db_conn();
    let res = &conn
        .query(
            "INSERT INTO persons (name, cult) VALUES ($1, $2) RETURNING id, name, cult;",
            &[&data.name, &data.cult],
        )
        .unwrap();
    let row = res.iter().next().unwrap();
    Person {
        id: row.get(0),
        name: row.get(1),
        cult: row.get(2)
    }
}
