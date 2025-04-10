use chrono::{NaiveDate, Weekday as wd};

fn middle_day(year: i32)->Option<wd>{
  let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);

  if is_leap {
    return None;
  }
  let date = NaiveDate::from_yo_opt(year, 184)?;
  Some(date.Weekday())
}


