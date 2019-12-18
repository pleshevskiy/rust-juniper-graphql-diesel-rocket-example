use juniper::FieldResult;

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

impl HeroObject {
    fn from_model(hero: &models::Hero) -> Self {
        HeroObject {
            id: hero.id.to_string(),
            name: hero.name.clone(),
            appears_in: hero.appears_in.clone(),
            home_planet: hero.home_planet.clone(),
        }
    }

    fn from_models(heroes: Vec<models::Hero>) -> Vec<Self> {
        heroes.iter()
            .map(|hero| HeroObject::from_model(hero))
            .collect()
    }
}



#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHeroInput {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

impl NewHeroInput {
    fn to_model(&self) -> NewHero {
        NewHero {
            name: self.name.clone(),
            home_planet: self.home_planet.clone(),
            appears_in: self.appears_in.clone(),
        }
    }
}


impl Context {
    pub fn get_hero(&self, id: &str) -> FieldResult<HeroObject> {
        let id: i32 = id.parse()?;
        let hero = models::Hero::get_hero(id, &self.connection)?;
        Ok(HeroObject::from_model(&hero))
    }

    pub fn all_heroes(&self) -> FieldResult<Vec<HeroObject>> {
        let heroes = models::Hero::get_all_heroes(&self.connection)?;
        Ok(HeroObject::from_models(heroes))
    }

    pub fn add_hero(&self, new_hero: NewHeroInput) -> FieldResult<HeroObject> {
        let hero = Hero::create(new_hero.to_model(), &self.connection)?;
        Ok(HeroObject::from_model(&hero))
    }
}
