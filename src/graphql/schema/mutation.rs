use juniper::FieldResult;

use crate::graphql::Context;
use crate::graphql::schema::{Hero, NewHero};


#[derive(Debug)]
pub struct Mutation {}

juniper::graphql_object!(Mutation: Context |&self| {
    field add_hero(&executor, new_hero: NewHero) -> FieldResult<Hero> {
        executor.context().add_hero(new_hero)
    }
});
