#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

use dotenv::dotenv;

mod app;
mod db;
mod graphql;

itconfig::config! {
    DATABASE_URL: String,

    ROCKET {
        static BASE_URL: String => "/",
    }
}

fn main() {
    dotenv().ok();

    app::init().launch();
}
