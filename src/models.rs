// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::NaiveDateTime;
use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}
