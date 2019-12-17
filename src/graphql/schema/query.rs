use juniper::FieldResult;

use crate::graphql::Context;
use crate::graphql::schema::Hero;


#[derive(Debug)]
pub struct Query {}

juniper::graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field hero(&executor, id: String) -> FieldResult<Hero> {
        executor.context().get_hero(&id)
    }

    field allHeroes(&executor) -> FieldResult<Vec<Hero>> {
        executor.context().all_heroes()
    }
});
