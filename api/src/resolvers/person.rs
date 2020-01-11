use actix_web::{web, Responder};

use crate::{models, types::Person};

pub fn get_person_list() -> impl Responder {
  let mut vec: Vec<Person> = Vec::new();
  models::person::get_person_all(&mut vec);
  web::Json(vec)
}

pub fn get_some_person() -> impl Responder {
  let mut vec: Vec<Person> = Vec::new();
  models::person::get_person_by_ids(&mut vec, vec![1,2,3]);
  web::Json(vec)
}

pub fn get_person(info: web::Path<i32>) -> impl Responder {
  let mut vec: Vec<Person> = Vec::new();
  models::person::get_person_by_id(&mut vec, info.into_inner());
  web::Json(vec)
}
