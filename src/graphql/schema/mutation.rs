use juniper::FieldResult;

use crate::graphql::schema::{HeroObject, NewHeroInput};
use crate::graphql::Context;

#[derive(Debug)]
pub struct Mutation {}

juniper::graphql_object!(Mutation: Context |&self| {
    field add_hero(&executor, input: NewHeroInput) -> FieldResult<HeroObject> {
        executor.context().add_hero(input)
    }
});
