use juniper;
use juniper::FieldError;
use juniper::FieldResult;
use juniper::graphql_value;
use crate::graphql::schema::Context;
use super::cults::Cult;


#[derive(Debug, Clone)]
pub struct Person {
  pub id: i32,
  pub name: String,
  pub cult: Option<i32>,
}

#[juniper::graphql_object(Context = Context)]
impl Person {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub async fn cult(&self, ctx: &Context) -> FieldResult<Cult> {
    match self.cult {
      Some(cult_id) => Ok(ctx.cult_data.cult_by_id(cult_id).await),
      None => Err(FieldError::new(
        "No cult",
        graphql_value!({ "internal_error": "No cult found" }),
      )),
    }
  }
}