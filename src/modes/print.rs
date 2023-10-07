use itertools::Itertools;

use crate::backend::api;

pub fn printable_entries(
    starting_date: chrono::NaiveDateTime,
    end_date: chrono::NaiveDateTime,
) -> Result<String, crate::errors::Error> {
    let response = api::get_entries_between_dates(starting_date, end_date)?;

    let mut answer = String::new();

    for (date, group) in &response
        .into_iter()
        .group_by(|api::EntryWithLabelsTuple(e, _, _)| e.timestamp.date())
    {
        let mut tmp_str = String::new();

        tmp_str.push_str(format!("## {}\n", &date.to_string()).as_str());

        for (time, group) in
            &group.group_by(|api::EntryWithLabelsTuple(e, _, _)| e.timestamp.time())
        {
            tmp_str.push_str(format!("### {}\n", time).as_str());
            for api::EntryWithLabelsTuple(entry, category_label, choice_label) in group {
                if let Some(cat) = category_label {
                    tmp_str.push_str(&cat);
                    if let Some(choice) = choice_label {
                        tmp_str.push_str(format!(" -> {}", choice).as_str());
                    }
                }
                if let Some(ref details) = entry.details {
                    if entry.category.is_some() {
                        tmp_str.push_str(" : ");
                    }
                    tmp_str.push_str(details);
                }
                tmp_str.push_str("  \n");
            }
        }
        answer.push_str(&tmp_str);
        answer.push('\n');
        answer.push('\n');
    }

    Ok(answer.trim().to_owned())
}
