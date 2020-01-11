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
        };
        vec.push(person);
    }
}

pub fn get_person_by_ids(vec: &mut Vec<Person>, ids: Vec<i32>) {
    let conn = get_db_conn();
    let mut in_list = "".to_string();
    // TODO: This could probably do with some tidying up
    for i in 0..ids.len() {
        let num = i + 1;
        let num_string = num.to_string();
        if i == 0 {
            in_list = format!("${}", &num_string);
        } else {
            in_list = format!("{}, ${}", in_list, &num_string);
        }
    }
    let query_string = format!(
        "SELECT person_id, person_name FROM persons WHERE person_id IN ({})",
        in_list
    );
    // let id_refs = ids.iter().map(|x| Box::new(x)).collect();
    println!("Atttempting query: \n {}", query_string);
    for row in &conn
        .query(&query_string, &[&ids[0],&ids[1],&ids[2]])//&ids[..])
        .unwrap()
    {
        let person = Person {
            person_id: row.get(0),
            person_name: row.get(1),
        };
        vec.push(person);
    }
}

pub fn get_person_by_id(vec: &mut Vec<Person>, id: i32) {
    // let pg_connection_string = env::var("DATABASE_URI").expect("need a db uri");
    // let conn = Connection::connect(&pg_connection_string[..], TlsMode::None).unwrap();
    let conn = get_db_conn();
    for row in &conn
        .query(
            "SELECT person_id, person_name FROM persons WHERE person_id = $1",
            &[&id],
        )
        .unwrap()
    {
        let person = Person {
            person_id: row.get(0),
            person_name: row.get(1),
        };
        vec.push(person);
    }
}
