#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

extern crate dotenv;
#[macro_use] extern crate juniper;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_derive_enum;

use dotenv::dotenv;
use rocket::response::content;
use rocket::State;

mod db;
mod graphql;

use crate::graphql::{Schema, create_schema, Context};
use crate::db::Connection;


#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    db: Connection,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context {
        connection: db
    })
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    db: Connection,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context {
        connection: db
    })
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .manage(db::connect())
        .manage(create_schema())
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}