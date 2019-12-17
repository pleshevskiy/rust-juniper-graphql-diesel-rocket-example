-- Your SQL goes here

CREATE TYPE episode_enum AS ENUM ('new_hope', 'empire', 'jedi');

CREATE TABLE heroes
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(128)   NOT NULL,
    appears_in  episode_enum[] NOT NULL DEFAULT '{}',
    home_planet VARCHAR(128)   NOT NULL
)
