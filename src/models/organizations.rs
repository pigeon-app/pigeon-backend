// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::schema::organizations;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use diesel_geography::types::GeogPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "organizations"]
pub struct Organizations {
    pub id: i64,
    pub display_name: String,
    pub is_recipient: bool,
    pub is_ready: bool,
    pub location: GeogPoint,
    pub created_at: NaiveDateTime,
}
