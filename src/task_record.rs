use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskRecord {
    date: String,
    length: String,
    task: String,
}

impl TaskRecord {
    // pub fn new(date: String, length: String, task: String) -> TaskRecord {
    //     TaskRecord {
    //         date: date,
    //         length: length,
    //         task: task,
    //     }
    // }

    pub fn for_today(length: String, task: String) -> TaskRecord {
        let now = Utc::now();
        let (_, this_year) = now.year_ce();
        let date =
            format!("{}-{:02}-{:02}", this_year, now.month(), now.day());

        TaskRecord { date, length, task }
    }
}
