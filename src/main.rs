#![feature(decl_macro)]
use rocket_contrib::serve::StaticFiles;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[macro_use] extern crate rocket;

fn main() {
    println!("Hello, world!");
    rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/webpack-starter/dist")))
            .launch();
}
