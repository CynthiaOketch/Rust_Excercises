use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {
    // Check if the year has an odd number of days
    let is_leap_year = chrono::NaiveDate::is_leap_year(&NaiveDate::from_ymd(year, 1, 1));

    let days_in_year = if is_leap_year { 366 } else { 365 };
    
    if days_in_year % 2 == 0 {
        // If the year has an even number of days, return None
        return None;
    }

    // Calculate the 183rd day (middle day) of the year
    let middle_day = 183;

    // Find the date of the middle day
    let middle_date = NaiveDate::from_ymd(year, 1, 1).with_day(middle_day).unwrap();

    // Get the weekday of the middle day
    let weekday = middle_date.weekday();
    
    Some(weekday)
}
