use crate::backend::establish_connection;
use crate::backend::schema;
use crate::models::{insertable as m_ins, queryable_or_selectable as m_qos};
use diesel::prelude::*;
use std::collections::BTreeMap;

pub mod patch;

// IDEA: maybe have functions return `queries`, so they can be more modular (e.g. apply a filter on the results of a query from another function)
// However, this is more abstraction, so only do it when it's actually necessary to refactor.

fn insert_entry(new_entry: m_ins::NewEntry) -> Result<(), diesel::result::Error> {
    use schema::entries::dsl::*;

    let mut connection = establish_connection();

    diesel::insert_into(entries)
        .values(&new_entry)
        .execute(&mut connection)?;
    Ok(())
}

// #[test]
// fn test_insert_entry() {
//     todo!()
// }

// pub fn get_categories_from_quiz_label(
//     quiz_label: &str,
// ) -> Result<Vec<m_qos::Category>, crate::errors::Error> {
//     use schema::{categories, quizzes, quizzes_to_categories};

//     use m_qos::Category;

//     let mut connection = establish_connection();

//     match quizzes::table
//         .filter(quizzes::label.eq(quiz_label))
//         .inner_join(
//             quizzes_to_categories::table
//                 .on(quizzes::label.eq(quizzes_to_categories::quiz_label)),
//         )
//         .inner_join(
//             categories::table.on(quizzes_to_categories::category_label.eq(categories::label)),
//         )
//         .select(categories::all_columns)
//         .load::<Category>(&mut connection)
//     {
//         Ok(vec_of_quizzes) => Ok(vec_of_quizzes),
//         Err(e) => Err(crate::errors::Error::DatabaseError(e)),
//     }
// }

/// Returns a map of categories to choices, from the choices in the quiz label.
pub fn get_categories_and_choices_from_quiz_label(
    quiz_label: &str,
) -> Result<BTreeMap<m_qos::Category, Option<Vec<m_qos::Choice>>>, crate::errors::Error> {
    use schema::{categories, choices, quizzes, quizzes_to_categories};

    let mut connection = establish_connection();

    let results: Vec<(m_qos::Category, Option<m_qos::Choice>)> = quizzes::table
        .inner_join(
            quizzes_to_categories::table
                .on(quizzes_to_categories::quiz_label.eq(quiz_label.to_string())),
        )
        .inner_join(
            categories::table.on(quizzes_to_categories::category_label.eq(categories::label)),
        )
        .left_outer_join(choices::table.on(categories::label.eq(choices::category_label)))
        .order((quizzes_to_categories::order, choices::shortcut)) // BUG: this doesn't actually sort by the order, only by the shortcut.
        .select((categories::all_columns, choices::all_columns.nullable()))
        .load::<_>(&mut connection)
        .expect("Error loading data");

    let mut actual_results = BTreeMap::<m_qos::Category, Option<Vec<m_qos::Choice>>>::new();

    for (cat, maybe_choice) in results {
        if let Some(c) = maybe_choice {
            if let Some(Some(cur_vec)) = actual_results.get(&cat) {
                let mut new_vec = cur_vec.clone();
                new_vec.push(c);
                actual_results.insert(cat, Some(new_vec));
            } else {
                actual_results.insert(cat, Some(vec![c]));
            }
        } else {
            actual_results.insert(cat, None);
        }
    }

    Ok(actual_results)
}

/// Returns the entries between the starting and ending dates, inclusive.
pub fn get_entries_between_dates(
    starting_date: chrono::NaiveDateTime,
    end_date: chrono::NaiveDateTime,
) -> Result<Vec<EntryWithLabelsTuple>, diesel::result::Error> {
    // TODO: review: maybe convert directly from a date instead of a datetime.

    use schema::{categories, choices, entries};

    let sd = starting_date;
    let ed = end_date;

    let mut connection = establish_connection();

    let results: Vec<EntryWithLabelsTuple> = entries::table
        .filter(entries::timestamp.le(sd))
        .filter(entries::timestamp.ge(ed))
        .inner_join(categories::table)
        .left_outer_join(choices::table)
        .order(entries::timestamp)
        .select((
            entries::all_columns,
            categories::label.nullable(),
            choices::label.nullable(),
        ))
        .load::<EntryCatLabelChoiceLabel>(&mut connection)
        .expect("Error loading entries between the dates.")
        .iter()
        .map(|(ent, cat, cho)| EntryWithLabelsTuple((*ent).clone(), cat.clone(), cho.clone()))
        .collect();

    Ok(results)
}

pub fn post_entry(
    category: Option<i32>,
    value: Option<i32>,
    details: Option<String>,
) -> Result<(), diesel::result::Error> {
    let new_entry = m_ins::NewEntry {
        category,
        value,
        details,
    };

    insert_entry(new_entry)
}

pub fn post_multiple_entries(
    entries: Vec<(Option<i32>, Option<i32>, Option<String>)>,
) -> Result<(), diesel::result::Error> {
    use schema::entries;
    let new_entries_obj: Vec<m_ins::NewEntry> = entries
        .iter()
        .map(|(cat_id, choice_id, comment)| m_ins::NewEntry {
            category: *cat_id,
            value: *choice_id,
            details: comment.clone(),
        })
        .collect();

    let mut connection = establish_connection();

    diesel::insert_into(entries::dsl::entries)
        .values(new_entries_obj)
        .execute(&mut connection)?;

    Ok(())
}

// pub fn get_timestamps_for_categories(
// ) -> Result<Vec<(String, Option<chrono::NaiveDateTime>)>, diesel::result::Error> {
//     use schema::{categories, entries};

//     let mut connection = establish_connection();

//     let results: Vec<(String, Option<chrono::NaiveDateTime>)> = categories::table
//         .left_join(entries::table)
//         .select((categories::label, entries::timestamp.nullable()))
//         .load::<_>(&mut connection)
//         .expect("Couldn't load data");

//     Ok(results)
// }

// pub fn get_latest_timestamp_for_categories(
// ) -> Result<Vec<(String, Option<chrono::NaiveDateTime>)>, diesel::result::Error> {
//     use schema::{categories, entries};

//     let mut connection = establish_connection();

//     let results: Vec<(String, Option<chrono::NaiveDateTime>)> = categories::table
//         .left_join(entries::table)
//         .group_by(categories::id)
//         .select((
//             categories::label,
//             diesel::dsl::max(entries::timestamp).nullable(),
//         ))
//         .load::<_>(&mut connection)
//         .expect("Couldn't load data");

//     Ok(results)
// }

/// Returns pairs of choice_label + timestamps for the choices that are shown in streaks.
pub fn get_timestamps_for_streaks_of_choices(
) -> Result<Vec<(String, Option<chrono::NaiveDateTime>)>, diesel::result::Error> {
    use schema::{choices, entries};

    let mut connection = establish_connection();

    let results: Vec<(String, Option<chrono::NaiveDateTime>)> = choices::table
        .filter(choices::show_in_streaks.eq(1))
        .left_join(entries::table)
        .select((choices::label, entries::timestamp.nullable()))
        .load::<_>(&mut connection)
        .expect("Couldn't load data");

    Ok(results)
}

/// Returns the latest timestamp for the choice with the given id.
pub fn get_latest_timestamp_for_choice(
    choice_id: i32,
) -> Result<Option<chrono::NaiveDateTime>, diesel::result::Error> {
    use schema::{choices, entries};

    let mut connection = establish_connection();

    let result: Option<chrono::NaiveDateTime> = choices::table
        .filter(choices::id.eq(choice_id))
        .left_join(entries::table)
        .select(diesel::dsl::max(entries::timestamp).nullable())
        .first(&mut connection)
        .expect("Couldn't load data");

    Ok(result)
}

// pub fn get_latest_timestamp_for_choices(
// ) -> Result<Vec<(String, Option<chrono::NaiveDateTime>)>, diesel::result::Error> {
//     use schema::{choices, entries};

//     let mut connection = establish_connection();

//     let results: Vec<(String, Option<chrono::NaiveDateTime>)> = choices::table
//         .left_join(entries::table)
//         .group_by(choices::id)
//         .select((
//             choices::label,
//             diesel::dsl::max(entries::timestamp).nullable(),
//         ))
//         .load::<_>(&mut connection)
//         .expect("Couldn't load data");

//     Ok(results)
// }

pub fn post_category(label: &str, prompt: &str) -> Result<(), diesel::result::Error> {
    use schema::categories;

    let new_category = m_ins::NewCategory {
        label: label.to_string(),
        prompt: prompt.to_string(),
        ..Default::default()
    };

    let mut connection = establish_connection();

    diesel::insert_into(categories::table)
        .values(&new_category)
        .execute(&mut connection)?;

    Ok(())
}

pub fn post_choice(
    label: &str,
    shortcut: &str,
    category_label: &str,
) -> Result<(), diesel::result::Error> {
    use schema::{categories, choices};

    // check that category exists:
    let mut connection = establish_connection();

    categories::table
        .filter(categories::label.eq(category_label))
        .select(categories::id)
        .first::<i32>(&mut connection)?;

    // TODO: check that shortcut is unique among the choices in the category.

    let new_choice = m_ins::NewChoice {
        label: label.to_string(),
        shortcut: shortcut.to_string(),
        category_label: category_label.to_string(),
        ..Default::default()
    };

    diesel::insert_into(choices::table)
        .values(&new_choice)
        .execute(&mut connection)?;

    Ok(())
}

pub fn post_quiz(label: &str) -> Result<(), diesel::result::Error> {
    use schema::quizzes;

    let new_quiz = m_ins::NewQuiz {
        label: label.to_string(),
        ..Default::default()
    };

    let mut connection = establish_connection();

    diesel::insert_into(quizzes::table)
        .values(&new_quiz)
        .execute(&mut connection)?;

    // This will return a unique constraint error if the quiz already exists.

    Ok(())
}

pub fn get_all_categories() -> Result<Vec<m_qos::Category>, diesel::result::Error> {
    use schema::categories;

    let mut connection = establish_connection();

    let results: Vec<m_qos::Category> = categories::table
        .load::<m_qos::Category>(&mut connection)
        .expect("Error loading categories");

    Ok(results)
}

pub fn get_choices_in_category(
    category_label: &str,
) -> Result<Vec<m_qos::Choice>, diesel::result::Error> {
    use schema::{categories, choices};

    let mut connection = establish_connection();

    let results: Vec<m_qos::Choice> = categories::table
        .filter(categories::label.eq(category_label))
        .inner_join(choices::table.on(categories::label.eq(choices::category_label)))
        .select(choices::all_columns)
        .load::<m_qos::Choice>(&mut connection)
        .expect("Error loading choices");

    Ok(results)
}

pub fn get_categories_in_quiz(
    quiz_label: &str,
) -> Result<Vec<m_qos::Category>, diesel::result::Error> {
    use schema::{categories, quizzes, quizzes_to_categories};

    let mut connection = establish_connection();

    let results: Vec<m_qos::Category> = quizzes::table
        .filter(quizzes::label.eq(quiz_label))
        .inner_join(
            quizzes_to_categories::table.on(quizzes::label.eq(quizzes_to_categories::quiz_label)),
        )
        .inner_join(
            categories::table.on(quizzes_to_categories::category_label.eq(categories::label)),
        )
        .select(categories::all_columns)
        .load::<m_qos::Category>(&mut connection)
        .expect("Error loading categories");

    Ok(results)
}

// type-aliases

type EntryCatLabelChoiceLabel = (m_qos::Entry, Option<String>, Option<String>);
pub struct EntryWithLabelsTuple(pub m_qos::Entry, pub Option<String>, pub Option<String>);
