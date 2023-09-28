pub mod queryable_or_selectable {
    use chrono::NaiveDateTime;
    use diesel::prelude::*;

    // #[derive(Queryable, Selectable)]
    // #[diesel(table_name = crate::backend::schema::quizzes)]
    // #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    // pub struct Quiz {
    //     pub id: i32,
    //     pub label: String,
    //     pub command: Option<String>,
    // }

    // #[derive(Queryable, Selectable)]
    // #[diesel(table_name = crate::backend::schema::quizzes_to_categories)]
    // #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    // pub struct QuizToCategory {
    //     pub id: i32,
    //     pub quiz_label: String,
    //     pub category_label: String,
    // }

    #[derive(Queryable, Selectable, Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
    #[diesel(table_name = crate::backend::schema::categories)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Category {
        pub id: i32,
        pub label: String,
        pub prompt: String,
        pub category_type: i32,
        pub disabled_bool: i32,
        pub extra_info: Option<String>,
        pub show_in_streaks: i32,
        pub reminder_timer_in_days: Option<i32>,
    }

    // #[derive(Queryable, Selectable)]
    // #[diesel(table_name = crate::backend::schema::category_types)]
    // #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    // pub struct CategoryType {
    //     pub id: i32,
    //     pub label: String,
    // }

    #[derive(Queryable, Selectable, Debug, Clone)]
    #[diesel(table_name = crate::backend::schema::entries)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Entry {
        pub id: i32,
        pub timestamp: NaiveDateTime,
        pub category: Option<i32>,
        pub value: Option<i32>,
        pub details: Option<String>,
    }

    #[derive(Queryable, Selectable, Debug, Clone)]
    #[diesel(table_name = crate::backend::schema::choices)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Choice {
        pub id: i32,
        pub label: String,
        pub shortcut: String,
        pub disabled_bool: i32,
        pub category_label: String,
        pub show_in_streaks: i32,
        pub reminder_timer_in_days: Option<i32>,
    }
}

pub mod insertable {
    use diesel::prelude::*;

    #[derive(Insertable)]
    #[diesel(table_name = crate::backend::schema::quizzes)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct NewQuiz {
        pub label: String,
        pub command: Option<String>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::backend::schema::quizzes_to_categories)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct NewQuizToCategory {
        pub quiz_label: String,
        pub category_label: String,
        pub order: i32,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::backend::schema::categories)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct NewCategory {
        pub label: String,
        pub prompt: String,
        pub category_type: i32,
        pub extra_info: Option<String>,
        pub show_in_streaks: Option<i32>,
        pub reminder_timer_in_days: Option<i32>,
    }

    // #[derive(Insertable)]
    // #[diesel(table_name = crate::backend::schema::category_types)]
    // #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    // pub struct NewCategoryType {
    //     pub label: String,
    // }

    #[derive(Insertable, Debug)]
    #[diesel(table_name = crate::backend::schema::entries)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct NewEntry {
        pub category: Option<i32>,
        pub value: Option<i32>,
        pub details: Option<String>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = crate::backend::schema::choices)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct NewChoice {
        pub label: String,
        pub shortcut: String,
        pub category_label: String,
        pub show_in_streaks: Option<i32>,
        pub reminder_timer_in_days: Option<i32>,
    }
}
