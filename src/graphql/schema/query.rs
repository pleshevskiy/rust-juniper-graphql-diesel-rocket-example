use juniper::FieldResult;

use crate::graphql::Context;
use crate::graphql::schema::HeroObject;


#[derive(Debug)]
pub struct Query {}

juniper::graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field hero(&executor, id: String) -> FieldResult<HeroObject> {
        executor.context().get_hero(&id)
    }

    field allHeroes(&executor) -> FieldResult<Vec<HeroObject>> {
        executor.context().all_heroes()
    }
});
