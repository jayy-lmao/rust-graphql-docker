use crate::types::Cult;
use crate::db::get_db_conn;

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
