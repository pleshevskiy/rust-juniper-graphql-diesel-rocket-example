mod query;
mod mutation;
mod hero;

use query::*;
use mutation::*;
pub use hero::*;


pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query { }, Mutation { })
}
