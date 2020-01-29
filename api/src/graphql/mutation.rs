use super::schema::Context;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
  // not really needed, but graphiql bug if this is empty…
  pub fn nothing(_name: String) -> i32 {
    0
  }
}