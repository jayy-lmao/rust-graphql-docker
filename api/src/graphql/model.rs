use juniper::Context as JuniperContext;

#[derive(Clone)]
pub(crate) struct Context {
}

impl JuniperContext for Context {}

impl Context {
    pub fn new() -> Self {
        Self {
        }
    }
}

pub(crate) struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn users(context: &Context, limit: Option<i32>) -> Vec<i32> {
      let vec = vec![1,2,3,4];
      vec
    }
}

pub(crate) struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    // not really needed, but graphiql bug if this is empty…
    pub fn nothing(name: String) -> i32 {
      0
    }
}

pub(crate) type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}
