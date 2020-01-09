use actix_web::{web, Responder};

use crate::{cult_repository,types::{Cult}};

pub fn get_cult_list() -> impl Responder {
    let mut vec:Vec<Cult> = Vec::new();
    cult_repository::get_cult_all(&mut vec);
    web::Json(vec)
}

pub fn get_cult(info: web::Path<i32>) -> impl Responder {
    let mut vec:Vec<Cult> = Vec::new();
    cult_repository::get_cult_by_id(&mut vec, info.into_inner());
    web::Json(vec)
}
