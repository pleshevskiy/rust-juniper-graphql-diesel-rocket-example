#[derive(Debug, PartialEq, DbEnum, Clone, GraphQLEnum)]
#[PgType = "episode_enum"]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}
