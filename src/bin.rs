// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use actix::{self, prelude::*};
use actix_web::server;
use dotenv::dotenv;
use env_logger;
use pigeon_backend::{
    backend_app,
    database::{connect, DbExecutor},
};

fn main() {
    dotenv().ok();
    let sys = actix::System::new("pigeon-backend");
    env_logger::init();

    let pool = connect();
    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    server::new(move || backend_app(address.clone()))
        .bind("127.0.0.1:8088")
        .expect("Couldn't bind to port 8088")
        .run();

    sys.run();
}
