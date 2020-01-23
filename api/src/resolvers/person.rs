use actix_web::{web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
  ids: Option<String>,
}

use crate::{models, types::Person};

pub fn get_person_list(info: web::Query<Info>) -> impl Responder {
  let mut vec: Vec<Person> = Vec::new();
  match &info.ids {
    Some(id_strings) => {
      let mut id_vector: Vec<i32> = vec![];
      for id_string in id_strings.split(",") {
        id_vector.push(id_string.parse().unwrap())
      }
      models::person::get_person_by_ids(&mut vec, id_vector);
    }
    None => {
      println!("here");
      models::person::get_person_all(&mut vec);
    }
  }
  web::Json(vec)
}

pub fn get_some_person(id_strings: web::Path<String>) -> impl Responder {
  let mut id_vector: Vec<i32> = vec![];
  for id_string in id_strings.split(",") {
    id_vector.push(id_string.parse().unwrap())
  }
  let mut vec: Vec<Person> = Vec::new();
  models::person::get_person_by_ids(&mut vec, id_vector);
  web::Json(vec)
}

pub fn get_person(info: web::Path<i32>) -> impl Responder {
  let mut vec: Vec<Person> = Vec::new();
  models::person::get_person_by_id(&mut vec, info.into_inner());
  web::Json(vec)
}
