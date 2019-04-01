// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::schema::{deliveries, items};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use diesel_geography::types::GeogPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "items"]
pub struct Items {
    pub id: i64,
    pub item: String,
    pub quantity: String,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "deliveries"]
pub struct Deliveries {
    pub id: i64,
    pub from_id: Option<i64>,
    pub to_id: Option<i64>,
}
