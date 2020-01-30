use juniper;
use juniper::FieldResult;
use crate::graphql::schema::Context;
use super::person::Person;


#[derive(Debug, Clone)]
pub struct Cult {
  pub id: i32,
  pub name: String,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
#[graphql(name="NewPerson", description="A creating a person!")]
pub struct NewCult {
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