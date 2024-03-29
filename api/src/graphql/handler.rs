use crate::data::{CultData, PersonData};
use crate::graphql::schema::{Context, Schema};
use actix_web::{error, web, Error, HttpResponse};
use juniper::http::{playground::playground_source, GraphQLRequest};
use std::sync::Arc;

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {

    // Context setup
    let person_data = PersonData::new();
    let cult_data = CultData::new();
    let ctx = Context::new(person_data, cult_data);

    // Execute
    let res = data.execute(&st, &ctx).await;
    let json = serde_json::to_string(&res).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

pub fn playground() -> HttpResponse {
    // I prefer playground but you can use graphiql as well
    let html = playground_source("http://localhost:8000/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
