use crate::type_defs::Person;

pub mod get_person_by_id;
pub mod get_persons_by_cult_id;

#[derive(Clone)]
pub struct PersonData {
  person_by_id: get_person_by_id::PersonLoader,
  persons_by_cult_id: get_persons_by_cult_id::PersonsLoader,
}

impl PersonData {
  pub fn new() -> PersonData {
    PersonData {
      person_by_id: get_person_by_id::get_loader(),
      persons_by_cult_id: get_persons_by_cult_id::get_loader(),
    }
  }
  pub async fn person_by_id(&self, id: i32) -> Person {
    self.person_by_id.load(id).await.unwrap()
  }
  pub async fn persons_by_cult_id(&self, id: i32) -> Vec<Person> {
    self.persons_by_cult_id.load(id).await.unwrap()
  }
}