use crate::graphql::model::{Context, Schema};
use crate::loaders::person_loader;
use actix_web::{error, web, Error, HttpResponse};
use dataloader::Loader;
use juniper::http::{playground::playground_source, GraphQLRequest};
use std::sync::Arc;

pub(super) async fn graphql(
  st: web::Data<Arc<Schema>>,
  data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
  let person_loader = Loader::new(person_loader::PersonBatcher);
  let ctx = Context::new(person_loader);

  let res = data.execute(&st, &ctx);
  let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(json),
  )
}

// pub(super) fn graphiql(opt: web::Data<Opt>) -> HttpResponse {
//     let html = graphiql_source(&format!("http://127.0.0.1:{}/graphql", opt.));
//     HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(html)
// }

pub(super) fn playground() -> HttpResponse {
  let html = playground_source("http://localhost:8000/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}
