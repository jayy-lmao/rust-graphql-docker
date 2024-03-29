extern crate postgres;
use crate::db::get_db_conn;
use crate::type_defs::Cult;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use async_trait::async_trait;
use std::collections::HashMap;

pub fn get_cult_by_ids(hashmap: &mut HashMap<i32, Cult>, ids: Vec<i32>) {
    let conn = get_db_conn();
    for row in &conn
        .query("SELECT id, name FROM cults WHERE id = ANY($1)", &[&ids])
        .unwrap()
    {
        let cult = Cult {
            id: row.get(0),
            name: row.get(1),
        };
        hashmap.insert(cult.id, cult);
    }
}

// pub fn create_cult(data: NewCult) -> Cult {
//     let conn = get_db_conn();
//     let res = &conn
//         .query(
//             "INSERT INTO cults (name, cult) VALUES ($1, $2) RETURNING id, name, cult;",
//             &[&data.name, &data.cult],
//         )
//         .unwrap();
//     let row = res.iter().next().unwrap();
//     Cult {
//         id: row.get(0),
//         name: row.get(1),
//         cult: row.get(2)
//     }
// }

pub struct CultBatcher;

#[async_trait]
impl BatchFn<i32, Cult> for CultBatcher {

    async fn load(&self, keys: &[i32]) -> HashMap<i32, Cult> {
        println!("load cult batch {:?}", keys);
        let mut cult_hashmap = HashMap::new();
        get_cult_by_ids(&mut cult_hashmap, keys.to_vec());
        cult_hashmap
    }
}

pub type CultLoader = Loader<i32, Cult, CultBatcher>;

pub fn get_loader() -> CultLoader {
    Loader::new(CultBatcher)
        .with_yield_count(100)
}
