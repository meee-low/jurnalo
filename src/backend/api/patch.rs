/// This file contains functions that modify the database.
use crate::backend::establish_connection;
use crate::backend::schema;
use crate::errors::Error;
use crate::models::insertable as m_ins;
use diesel::prelude::*;

pub fn link_category_to_quiz(category: &str, quiz: &str) -> Result<(), Error> {
    use schema::{categories, quizzes, quizzes_to_categories};

    let mut connection = establish_connection(None);

    // confirm that category exists in the database:
    categories::table
        .filter(categories::label.eq(category))
        .select(categories::id)
        .first::<i32>(&mut connection)?;

    // confirm that quiz exists in the database and get the order of the last category in the quiz:
    let (_quiz_id, order) = match quizzes::table
        .filter(quizzes::label.eq(quiz))
        .left_join(
            quizzes_to_categories::table.on(quizzes_to_categories::quiz_label.eq(quizzes::label)),
        )
        .group_by(quizzes::id)
        .select((
            quizzes::id,
            diesel::dsl::max(quizzes_to_categories::order).nullable(),
        ))
        .first::<(i32, Option<i32>)>(&mut connection)
    {
        Ok((id, order)) => (id, order),
        Err(e) => return Err(Error::DatabaseError(e)),
    };

    // TODO: check if the quiz already contains the category

    let next_order = order.unwrap_or(0) + 1;

    // insert the new link:
    let new_link = m_ins::NewQuizToCategory {
        quiz_label: quiz.to_owned(),
        category_label: category.to_owned(),
        order: next_order,
    };

    match diesel::insert_into(quizzes_to_categories::table)
        .values(&new_link)
        .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}

pub fn unlink_category_from_quiz(category: &str, quiz: &str) -> Result<(), Error> {
    use schema::quizzes_to_categories;

    let mut connection = establish_connection(None);

    // confirm that category exists in the database:
    quizzes_to_categories::table
        .filter(
            quizzes_to_categories::category_label
                .eq(category)
                .and(quizzes_to_categories::quiz_label.eq(quiz)),
        )
        .select(quizzes_to_categories::id)
        .first::<i32>(&mut connection)?;

    // TODO: confirm that the link exists in fact

    // delete the link:
    match diesel::delete(
        quizzes_to_categories::table.filter(
            quizzes_to_categories::category_label
                .eq(category)
                .and(quizzes_to_categories::quiz_label.eq(quiz)),
        ),
    )
    .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}

pub fn disable_choice(category: &str, choice: &str) -> Result<(), Error> {
    use schema::choices;

    let mut connection = establish_connection(None);

    // confirm that choice exists in the database:
    choices::table
        .filter(
            choices::label
                .eq(choice)
                .and(choices::category_label.eq(category)),
        )
        .select(choices::id)
        .first::<i32>(&mut connection)?;

    // disable the choice:
    match diesel::update(choices::table.filter(choices::label.eq(choice)))
        .set(choices::disabled_bool.eq(1))
        .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}

pub fn disable_category(category: &str) -> Result<(), Error> {
    use schema::categories;

    let mut connection = establish_connection(None);

    // confirm that category exists in the database:
    categories::table
        .filter(categories::label.eq(category))
        .select(categories::id)
        .first::<i32>(&mut connection)?;

    // disable the category:
    match diesel::update(categories::table.filter(categories::label.eq(category)))
        .set(categories::disabled_bool.eq(1))
        .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}

pub fn toggle_show_in_streaks_for_choice(category: &str, choice: &str) -> Result<(), Error> {
    use schema::choices;

    let mut connection = establish_connection(None);

    // confirm that choice exists in the database:
    choices::table
        .filter(
            choices::label
                .eq(choice)
                .and(choices::category_label.eq(category)),
        )
        .select(choices::id)
        .first::<i32>(&mut connection)?;

    // toggle the show_in_streaks field:
    match diesel::update(choices::table.filter(choices::label.eq(choice)))
        .set(
            choices::show_in_streaks.eq(diesel::dsl::sql::<diesel::sql_types::Integer>(
                "CASE WHEN show_in_streaks = 1 THEN 0 ELSE 1 END",
            )),
        )
        .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}

pub fn change_timer_for_choice(choice: &str, new_timer: Option<i32>) -> Result<(), Error> {
    use schema::choices;

    let mut connection = establish_connection(None);

    // confirm that choice exists in the database:
    choices::table
        .filter(choices::label.eq(choice))
        .select(choices::id)
        .first::<i32>(&mut connection)?;

    // change the timer:
    match diesel::update(choices::table.filter(choices::label.eq(choice)))
        .set(choices::reminder_timer_in_days.eq(new_timer))
        .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}

pub fn move_last_entry_to_yesterday() -> Result<(), Error> {
    use schema::entries;

    let mut connection = establish_connection(None);

    // get the timestamp of the last entry:
    let last_entry_timestamp = match entries::table
        .order(entries::timestamp.desc())
        .select(entries::timestamp)
        .first::<chrono::NaiveDateTime>(&mut connection)
    {
        Ok(id) => id,
        Err(e) => return Err(Error::DatabaseError(e)),
    };

    let back_one_day = last_entry_timestamp - chrono::Duration::days(1);

    // move the last entry to yesterday:
    match diesel::update(entries::table.filter(entries::timestamp.eq(last_entry_timestamp)))
        .set(entries::timestamp.eq(back_one_day))
        .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}

pub fn rename_category(category: &str, new_name: &str) -> Result<(), Error> {
    use schema::categories;

    let mut connection = establish_connection(None);

    // confirm that category exists in the database:
    categories::table
        .filter(categories::label.eq(category))
        .select(categories::id)
        .first::<i32>(&mut connection)?;

    // confirm that the new name is not already taken:
    match categories::table
        .filter(categories::label.eq(new_name))
        .select(categories::id)
        .first::<i32>(&mut connection)
    {
        Ok(_) => return Err(Error::CategoryAlreadyExists(new_name.to_owned())),
        Err(diesel::result::Error::NotFound) => (),
        Err(e) => return Err(Error::DatabaseError(e)),
    }

    // rename the category:
    match diesel::update(categories::table.filter(categories::label.eq(category)))
        .set(categories::label.eq(new_name))
        .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}

pub fn rename_choice(category: &str, choice: &str, new_name: &str) -> Result<(), Error> {
    use schema::choices;

    let mut connection = establish_connection(None);

    // confirm that choice exists in the database:
    choices::table
        .filter(
            choices::label
                .eq(choice)
                .and(choices::category_label.eq(category)),
        )
        .select(choices::id)
        .first::<i32>(&mut connection)?;

    // confirm that the new name is not already taken within the category:
    match choices::table
        .filter(
            choices::label
                .eq(new_name)
                .and(choices::category_label.eq(category)),
        )
        .select(choices::id)
        .first::<i32>(&mut connection)
    {
        Ok(_) => return Err(Error::ChoiceAlreadyExists(new_name.to_owned())),
        Err(diesel::result::Error::NotFound) => (),
        Err(e) => return Err(Error::DatabaseError(e)),
    }

    // rename the choice:
    match diesel::update(choices::table.filter(choices::label.eq(choice)))
        .set(choices::label.eq(new_name))
        .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}

pub fn rename_quiz(quiz: &str, new_name: &str) -> Result<(), Error> {
    use schema::quizzes;

    let mut connection = establish_connection(None);

    // confirm that quiz exists in the database:
    quizzes::table
        .filter(quizzes::label.eq(quiz))
        .select(quizzes::id)
        .first::<i32>(&mut connection)?;

    // confirm that the new name is not already taken:
    match quizzes::table
        .filter(quizzes::label.eq(new_name))
        .select(quizzes::id)
        .first::<i32>(&mut connection)
    {
        Ok(_) => return Err(Error::QuizAlreadyExists(new_name.to_owned())),
        Err(diesel::result::Error::NotFound) => (),
        Err(e) => return Err(Error::DatabaseError(e)),
    }

    // rename the quiz:
    match diesel::update(quizzes::table.filter(quizzes::label.eq(quiz)))
        .set(quizzes::label.eq(new_name))
        .execute(&mut connection)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::DatabaseError(e)),
    }
}
