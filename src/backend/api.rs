use crate::backend::establish_connection;
use crate::backend::schema;
use crate::models::{insertable as m_ins, queryable_or_selectable as m_qos};
use diesel::prelude::*;
use std::collections::BTreeMap;

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
        .order_by(choices::shortcut)
        .select((categories::all_columns, choices::all_columns.nullable()))
        .load::<(m_qos::Category, Option<m_qos::Choice>)>(&mut connection)
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
        }
    }

    Ok(actual_results)
}

pub fn get_entries_between_dates(
    starting_date: chrono::NaiveDateTime,
    end_date: chrono::NaiveDateTime,
) -> Result<Vec<m_qos::Entry>, diesel::result::Error> {
    // TODO: review: maybe convert directly from a date instead of a datetime.

    use schema::entries;

    let sd = starting_date;
    let ed = end_date;

    let mut connection = establish_connection();

    let results: Vec<m_qos::Entry> = entries::table
        .filter(entries::timestamp.le(sd))
        .filter(entries::timestamp.ge(ed))
        .order(entries::timestamp)
        .select(entries::all_columns)
        .load::<m_qos::Entry>(&mut connection)
        .expect("Error loading entries between the dates.");

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
