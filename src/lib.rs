extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;

pub mod api;
pub mod db;
pub mod handlers;
pub mod models;
pub mod schema;
