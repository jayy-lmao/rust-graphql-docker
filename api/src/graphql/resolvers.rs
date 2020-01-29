use crate::data::cult::CultData;
use crate::data::person::PersonData;
use juniper;
use juniper::FieldError;
use juniper::FieldResult;
use juniper::graphql_value;

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

#[derive(Debug, Clone)]
pub struct Cult {
  pub id: i32,
  pub name: String,
}

#[juniper::graphql_object(Context = Context)]
impl Cult {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub async fn members(&self, ctx: &Context) -> FieldResult<Vec<Person>> {
    Ok(ctx.person_data.persons_by_cult_id(self.id).await)
  }
}

#[derive(Clone)]
pub struct Context {
  person_data: PersonData,
  cult_data: CultData,
}

impl juniper::Context for Context {}

impl Context {
  pub fn new(person_data: PersonData, cult_data: CultData) -> Self {
    Self {
      person_data,
      cult_data,
    }
  }
}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
  async fn person_by_id(context: &Context, id: i32) -> FieldResult<Person> {
    Ok(context.person_data.person_by_id(id).await)
  }
  async fn cult_by_id(context: &Context, id: i32) -> FieldResult<Cult> {
    Ok(context.cult_data.cult_by_id(id).await)
  }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
  // not really needed, but graphiql bug if this is empty…
  pub fn nothing(_name: String) -> i32 {
    0
  }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
  Schema::new(Query {}, Mutation {})
}
