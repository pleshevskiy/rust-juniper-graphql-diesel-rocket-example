use juniper::{FieldResult, FieldError};

use crate::graphql::Context;
use crate::db::enums::Episode;
use crate::db::models;
use crate::db::models::Hero;
use crate::db::models::hero::NewHero;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct HeroObject {
    pub id: String,
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}


impl From<Hero> for HeroObject {
    fn from(hero: Hero) -> Self {
        HeroObject {
            id: hero.id.to_string(),
            name: hero.name.clone(),
            appears_in: hero.appears_in.clone(),
            home_planet: hero.home_planet.clone(),
        }
    }
}


#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHeroInput {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}


impl Context {
    pub fn get_hero(&self, id: &str) -> FieldResult<HeroObject> {
        let id: i32 = id.parse()?;
        let hero = models::Hero::get_hero(id, &self.connection)?;
        Ok(hero.into())
    }

    pub fn all_heroes(&self) -> FieldResult<Vec<HeroObject>> {
        let heroes = models::Hero::get_all_heroes(&self.connection)?;
        let hero_objects = heroes.iter()
            .map(|hero| HeroObject::from(hero.clone()))
            .collect();
        Ok(hero_objects)
    }

    pub fn add_hero(&self, input: NewHeroInput) -> FieldResult<HeroObject> {
        let new_hero = NewHero {
            name: input.name.clone(),
            appears_in: input.appears_in.clone(),
            home_planet: input.home_planet.clone(),
        };

        let hero = Hero::create(new_hero, &self.connection)
            .map_err(|_| FieldError::from("Cannot create hero"))?;
        Ok(hero.into())
    }
}
