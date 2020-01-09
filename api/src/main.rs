use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

mod person;
mod cult;

#[derive(Serialize)]
pub struct Cult {
    pub id: i32,
    pub name: String
}

fn get_cult_list() -> impl Responder {
    let mut vec:Vec<Cult> = Vec::new();
    cult::get_cult_all(&mut vec);
    web::Json(vec)
}

fn get_cult(info: web::Path<i32>) -> impl Responder {
    let mut vec:Vec<Cult> = Vec::new();
    cult::get_cult_by_id(&mut vec, info.into_inner());
    web::Json(vec)
}
 
#[derive(Serialize)]
pub struct Person {
    pub person_id: i32,
    pub person_name: String
}
 
fn get_person_list() -> impl Responder {
    let mut vec:Vec<Person> = Vec::new();
    person::get_person_all(&mut vec);
    web::Json(vec)
}

// Use i32 for int and serial in postgresql
fn get_person(info: web::Path<i32>) -> impl Responder {
    let mut vec:Vec<Person> = Vec::new();
    person::get_person_by_id(&mut vec, info.into_inner());
    web::Json(vec)
}
 
fn greet() -> impl Responder {
    "Yo there"
}

fn main() {
    println!("Server starting...");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/hello")
                .route(web::get().to(greet)))
            .service(web::resource("/persons/{person_id}")
                .route(web::get().to(get_person)))
                // .route(web::delete().to(delete_person))
                // .route(web::put().to(update_person)))
            .service(web::resource("/cults/{cult_id}")
                .route(web::get().to(get_cult)))
            .service(web::resource("/cults")
                .route(web::get().to(get_cult_list)))
            .service(web::resource("/persons")
                .route(web::get().to(get_person_list)))
                // .route(web::post().to(create_person)))
    })
        .bind("0.0.0.0:8000")
        .unwrap()
        .run()
        .unwrap();
}
