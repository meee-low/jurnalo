// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Integer,
        label -> Text,
        prompt -> Text,
        category_type -> Integer,
        disabled_bool -> Integer,
        extra_info -> Nullable<Text>,
    }
}

diesel::table! {
    category_types (id) {
        id -> Integer,
        label -> Text,
    }
}

diesel::table! {
    choices (id) {
        id -> Integer,
        label -> Text,
        shortcut -> Text,
        disabled_bool -> Integer,
        category_label -> Text,
    }
}

diesel::table! {
    entries (id) {
        id -> Integer,
        timestamp -> Timestamp,
        category -> Nullable<Integer>,
        value -> Nullable<Integer>,
        details -> Nullable<Text>,
    }
}

diesel::table! {
    quizzes (id) {
        id -> Integer,
        label -> Text,
        command -> Nullable<Text>,
    }
}

diesel::table! {
    quizzes_to_categories (id) {
        id -> Integer,
        quiz_id -> Integer,
        category_id -> Integer,
    }
}

diesel::joinable!(categories -> category_types (category_type));
diesel::joinable!(entries -> categories (category));
diesel::joinable!(entries -> choices (value));
diesel::joinable!(quizzes_to_categories -> categories (category_id));
diesel::joinable!(quizzes_to_categories -> quizzes (quiz_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    category_types,
    choices,
    entries,
    quizzes,
    quizzes_to_categories,
);
