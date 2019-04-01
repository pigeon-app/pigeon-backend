table! {
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    deliveries (id) {
        id -> Int8,
        from_id -> Int8,
        to_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    items (id) {
        id -> Int8,
        item -> Varchar,
        quantity -> Varchar,
        expires_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    organizations (id) {
        id -> Int8,
        display_name -> Varchar,
        is_recipient -> Bool,
        is_ready -> Bool,
        location -> Geography,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    users (id) {
        id -> Int8,
        display_name -> Nullable<Varchar>,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    deliveries,
    items,
    organizations,
    spatial_ref_sys,
    users,
);
