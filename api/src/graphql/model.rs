use crate::loaders::person_loader::PersonBatcher;
use crate::models::cult;
use dataloader::Loader;
use juniper;
use futures::{executor, future::Future};

#[derive(Debug, Clone)]
pub struct Person {
    pub person_id: i32,
    pub person_name: String,
    pub cult: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Person {
  pub fn id(&self) -> i32 {
    self.person_id
  }

  pub fn name(&self) -> &str {
    self.person_name.as_str()
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

  pub async fn members(context: &Context) -> Vec<Person> {
      let mut vec = Vec::new();
      for i in 1..3 {
        vec.push(person_by_id(context, i).await);
      }
      vec
  }
}

#[derive(Clone)]
pub struct Context {
    person_loader: Loader<i32, Person, (), PersonBatcher>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(person_loader: Loader<i32, Person, (), PersonBatcher>) -> Self {
        Self { person_loader }
    }
}

pub struct Query;

async fn person_by_id(ctx: &Context, id: i32) -> Person {
    let res = ctx.person_loader.load(id);
    res.await.unwrap()
}

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn users(context: &Context, limit: Option<i32>) -> Vec<i32> {
        let vec = vec![1, 2, 3, 4];
        vec
    }
    async fn person_by_id(context: &Context, id: i32) -> Person {
        person_by_id(context, id).await
    }
    async fn cults() -> Vec<Cult> {
        let mut vec = Vec::new();
        cult::get_cult_all(&mut vec);
        vec
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    // not really needed, but graphiql bug if this is empty…
    pub fn nothing(name: String) -> i32 {
        0
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
