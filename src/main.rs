// Workaround for #50504
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

mod models;
mod schema;
mod handlers;

use std::env;

use ::actix::prelude::*;
use actix_web::{server, App, http::Method, http::StatusCode, HttpRequest, HttpResponse};
use actix_web::middleware::{Middleware, Started, Response};
use actix_web::middleware::Logger;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use self::handlers::{get_example, post_example};

pub struct DbExecutor(SqliteConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

impl DbExecutor {
    pub fn new() -> DbExecutor {
        DbExecutor(establish_connection())
    }
}

pub struct AppState {
    db: Addr<DbExecutor>,
}

fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix::System::new("rustgbg-web");

    // Start 3 parallel db executors
    let addr = SyncArbiter::start(3, || DbExecutor::new());

    // Start http server
    server::new(move || { App::with_state(AppState { db: addr.clone() })
        .middleware(Logger::default())
        .resource("/example/{id}", |r| {
            r.method(Method::GET).with(get_example);
        })
        .resource("/examples", |r| {
            r.method(Method::POST).with(post_example);
        })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    let _ = sys.run();
}