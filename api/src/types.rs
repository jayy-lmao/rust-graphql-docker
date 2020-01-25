
pub struct Person {
  pub person_id: i32,
  pub person_name: String,
  pub cult: Option<i32>,
}

pub struct Cult {
  pub id: i32,
  pub name: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewPerson {
    pub name: String,
    pub cult: Option<i32>
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewCult {
  pub name: String,
}