use std::collections::HashMap;
use chrono::{DateTime, Utc, Datelike};
pub use json::JsonValue;


pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut week_map = HashMap::new();

    if let JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
                if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                    let date = date.with_timezone(&Utc);
                    let iso_week = date.iso_week();
                    let week_key = format!("{}-W{}", iso_week.year(), iso_week.week());
                    *week_map.entry(week_key).or_insert(0) += 1;
                }
            }
        }
    }

    week_map
}


pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut author_map = HashMap::new();

    if let JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(author_login) = commit["author"]["login"].as_str() {
                *author_map.entry(author_login.to_string()).or_insert(0) += 1;
            }
        }
    }

    author_map
}
