// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate diesel;

pub mod database;
pub mod models;
pub mod schema;

use actix::prelude::*;
use actix_web::{App, HttpRequest, Responder};
use crate::database::DbExecutor;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn greet(req: &HttpRequest<AppState>) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

pub fn backend_app(db: Addr<DbExecutor>) -> App<AppState> {
    App::with_state(AppState { db })
        .prefix("/mapi/v1")
        .resource("/", |r| r.f(greet))
        .resource("/{name}", |r| r.f(greet))
}
