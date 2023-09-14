use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i32,
    pub label: String,
    pub prompt: String,
    pub category_type: i32,
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
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub entry_number: i32,
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
