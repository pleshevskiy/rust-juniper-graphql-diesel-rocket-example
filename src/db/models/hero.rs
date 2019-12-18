use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::db::enums::Episode;
pub use crate::db::schema::heroes;

#[derive(AsChangeset, Queryable, Identifiable, Debug, Clone, PartialEq)]
#[table_name = "heroes"]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

#[derive(Debug, Insertable)]
#[table_name = "heroes"]
pub struct NewHero {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

impl Hero {
    pub fn create(
        new_hero: NewHero,
        connection: &PgConnection,
    ) -> QueryResult<Hero> {
        diesel::insert_into(heroes::table)
            .values(&new_hero)
            .get_result(connection)
    }

    pub fn get_hero(id: i32, connection: &PgConnection) -> QueryResult<Hero> {
        heroes::table.find(id).first::<Hero>(connection)
    }

    pub fn get_all_heroes(
        connection: &PgConnection,
    ) -> QueryResult<Vec<Hero>> {
        heroes::table.order(heroes::id).load::<Hero>(connection)
    }

    pub fn update(id: i32, hero: Hero, connection: &PgConnection) -> bool {
        diesel::update(heroes::table.find(id))
            .set(&hero)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
    }
}
