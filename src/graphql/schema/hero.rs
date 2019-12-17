use juniper::FieldResult;

use crate::graphql::Context;
use crate::db::enums::Episode;
use crate::db::models;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Hero {
    pub id: String,
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

impl Hero {
    fn from_model(hero: &models::Hero) -> Self {
        Hero {
            id: hero.id.unwrap().to_string(),
            name: hero.name.clone(),
            appears_in: hero.appears_in.clone(),
            home_planet: hero.home_planet.clone(),
        }
    }

    fn from_models(heroes: Vec<models::Hero>) -> Vec<Self> {
        heroes.iter()
            .map(|hero| Hero::from_model(hero))
            .collect()
    }
}



#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHero {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

impl NewHero {
    fn to_model(&self) -> models::Hero {
        models::Hero {
            id: None,
            name: self.name.clone(),
            home_planet: self.home_planet.clone(),
            appears_in: self.appears_in.clone(),
        }
    }
}


impl Context {
    pub fn get_hero(&self, id: &str) -> FieldResult<Hero> {
        let id: i32 = id.parse()?;
        let hero = models::Hero::get_hero(id, &self.connection)?;
        Ok(Hero::from_model(&hero))
    }

    pub fn all_heroes(&self) -> FieldResult<Vec<Hero>> {
        let heroes = models::Hero::get_all_heroes(&self.connection)?;
        Ok(Hero::from_models(heroes))
    }

    pub fn add_hero(&self, new_hero: NewHero) -> FieldResult<Hero> {
        let hero = models::Hero::create(new_hero.to_model(), &self.connection)?;
        Ok(Hero::from_model(&hero))
    }
}
