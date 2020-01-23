#[macro_use]
extern crate juniper;

mod db;
mod schema;
mod models;
mod resolvers;
mod types;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::GraphQLRequest;
use juniper::http::playground::playground_source;
use std::io;
use std::sync::Arc;
use crate::schema::{create_schema, Schema};

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn playground() -> HttpResponse {
    let html = playground_source("http://localhost:8000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn main() -> io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
        App::new().data(schema.clone()).service(
            web::resource("/graphql")
                .route(web::post().to_async(graphql))
                .route(web::get().to(playground)),
        )
    })
    .bind("0.0.0.0:8000")?
    .run()
}
