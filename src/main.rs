#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate itconfig;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

use dotenv::dotenv;

mod app;
mod config;
mod db;
mod graphql;

config! {
    DATABASE_URL: String,

    ROCKET {
        BASE_URL: String => "/".to_string(),
    }
}

fn main() {
    dotenv().ok();

    app::init().launch();
}
