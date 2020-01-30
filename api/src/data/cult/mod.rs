use crate::type_defs::{Cult, NewCult};

pub mod get_cult_by_id;
pub mod create_cult;
use get_cult_by_id::{get_loader, CultLoader};

#[derive(Clone)]
pub struct CultData {
  cult_by_id: CultLoader,
}

impl CultData {
  pub fn new() -> CultData {
    CultData {
      cult_by_id: get_loader(),
    }
  }
  pub async fn cult_by_id(&self, id: i32) -> Cult {
    self.cult_by_id.load(id).await.unwrap()
  }
  pub async fn create_cult(&self, data: NewCult) -> Cult {
    create_cult::create_cult(data)
  }
}