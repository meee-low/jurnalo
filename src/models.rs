pub mod queryable_or_selectable {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::batteries)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Battery {
        pub id: i32,
        pub label: String,
        pub command: Option<String>,
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::batteries_to_categories)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct BatteryToCategory {
        pub id: i32,
        pub battery_id: i32,
        pub category_id: i32,
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::categories)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Category {
        pub id: i32,
        pub label: String,
        pub prompt: String,
        pub category_type: i32,
        pub disabled_bool: i32,
        pub extra_info: Option<String>,
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::category_option)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct CategoryOption {
        pub id: i32,
        pub category_id: i32,
        pub option_id: i32,
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::category_types)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct CategoryType {
        pub id: i32,
        pub label: String,
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::entries)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Entry {
        pub id: i32,
        pub timestamp: NaiveDateTime,
        pub category: Option<i32>,
        pub value: Option<i32>,
        pub details: Option<String>,
    }

    #[derive(Queryable, Selectable)]
    #[diesel(table_name = crate::schema::options)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct DBOption {
        pub id: i32,
        pub label: String,
        pub shortcut: String,
    }
}

pub mod insertable {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;

    #[derive(Insertable)]
    #[diesel(table_name = crate::schema::batteries)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Battery {
        pub label: String,
        pub command: Option<String>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::schema::batteries_to_categories)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct BatteryToCategory {
        pub battery_id: i32,
        pub category_id: i32,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::schema::categories)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Category {
        pub label: String,
        pub prompt: String,
        pub category_type: i32,
        pub extra_info: Option<String>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::schema::category_option)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct CategoryOption {
        pub category_id: i32,
        pub option_id: i32,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::schema::category_types)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct CategoryType {
        pub label: String,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::schema::entries)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Entry {
        pub timestamp: NaiveDateTime,
        pub category: Option<i32>,
        pub value: Option<i32>,
        pub details: Option<String>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::schema::options)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct DBOption {
        pub label: String,
        pub shortcut: String,
    }
}
