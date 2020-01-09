use actix_web::{web, Responder};

use crate::{person_repository,types::{Person}};

pub fn get_person_list() -> impl Responder {
    let mut vec:Vec<Person> = Vec::new();
    person_repository::get_person_all(&mut vec);
    web::Json(vec)
}

pub fn get_person(info: web::Path<i32>) -> impl Responder {
    let mut vec:Vec<Person> = Vec::new();
    person_repository::get_person_by_id(&mut vec, info.into_inner());
    web::Json(vec)
}
