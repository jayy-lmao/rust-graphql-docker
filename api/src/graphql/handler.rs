use crate::graphql::resolvers::{Context, Schema};
use actix_web::{error, web, Error, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};
use std::sync::Arc;
use crate::data::cult::CultData;
use crate::data::person::PersonData;

pub(super) async fn graphql(
  st: web::Data<Arc<Schema>>,
  data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
  let mut rt = futures::executor::LocalPool::new();
  let person_data = PersonData::new();
  let cult_data = CultData::new();
  let ctx = Context::new(person_data, cult_data);
  let f = data.execute_async(&st, &ctx);
  let res = rt.run_until(f);
  let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(json),
  )
}

pub(super) fn playground() -> HttpResponse {
  let html = playground_source("http://localhost:8000/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}
