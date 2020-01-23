use crate::db::get_db_conn;
use crate::types::{Cult, NewCult};
use juniper::FieldResult;

pub fn get_cult_all(vec: &mut Vec<Cult>) {
    let conn = get_db_conn();
    for row in &conn.query("SELECT id, name FROM cults", &[]).unwrap() {
        let cult = Cult {
            id: row.get(0),
            name: row.get(1),
        };
        vec.push(cult);
    }
}

pub fn get_cult_by_id(vec: &mut Vec<Cult>, id: i32) {
    let conn = get_db_conn();
    for row in &conn
        .query("SELECT id, name FROM cults WHERE id = $1", &[&id])
        .unwrap()
    {
        let cult = Cult {
            id: row.get(0),
            name: row.get(1),
        };
        vec.push(cult);
    }
}

pub fn create_cult(data: NewCult) -> FieldResult<Option<Cult>> {
    let conn = get_db_conn();
    let res = &conn.query(
        "INSERT INTO cults (name) VALUES ($1) RETURNING id, name;",
        &[&data.name],
    );
    // An example of untidy, but effective, error handling.
    match res {
        Ok(r) => {
            let row = r.iter().next().unwrap();
            Ok(Some(Cult {
                id: row.get(0),
                name: row.get(1),
            }))
        }
        Err(e) => Err(e)?,
    }
}
