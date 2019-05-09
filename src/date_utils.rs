//! Helper functions for common tasks upon date variables.

use chrono::NaiveDate;
use chrono::prelude::*;

/// Gets current date as chrono::NaiveDate.
pub fn now() -> NaiveDate {
    let d = Local::now().date();
    NaiveDate::from_ymd(d.year(), d.month(), d.day())
}

pub fn next_month(date: NaiveDate) -> NaiveDate {
    let month = date.month();
    let year = date.year();
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    NaiveDate::from_ymd(next_year, next_month, 1)
}

pub fn prev_month(date: NaiveDate) -> NaiveDate {
    let month = date.month();
    let year = date.year();
    let prev_month = if month == 1 { 12 } else { month - 1 };
    let prev_year = if month == 1 { year - 1 } else { year };
    NaiveDate::from_ymd(prev_year, prev_month, 1)
}

/// Gets last day of given month in given year.
/// Algorithm:
///  - get first day of next month
///  - get one day before - certain to be last day of given month
///  - iterate in simple integer for loop from 1 to last day of month
///  - build dates from numbers
/// Note we have to handle december specifically!
pub fn last_day(year: i32, month: u32) -> u32 {
    let next_month = if month == 12 { 1 } else { month + 1 };
    NaiveDate::from_ymd(year, next_month, 1).pred().day()
}

/// Get vector of days in given month and year.
pub fn get_month_days(year: i32, month: u32) -> Vec<NaiveDate> {

    let last_day = last_day(year, month);

    let mut days: Vec<NaiveDate> = Vec::with_capacity(5);

    for day in 1..(last_day+1) {
        days.push(NaiveDate::from_ymd(year, month, day));
    }

    let days = days;

    days
}

/// Map chrono::Weekday to string containing Czech name of given weekday.
pub fn get_weekday_name(i: chrono::Weekday) -> String {
    match i {
        Weekday::Mon => String::from("Pondělí"),
        Weekday::Tue => String::from("Úterý"),
        Weekday::Wed => String::from("Středa"),
        Weekday::Thu => String::from("Čtvrtek"),
        Weekday::Fri => String::from("Pátek"),
        Weekday::Sat => String::from("Sobota"),
        Weekday::Sun => String::from("Neděle")
    }
}

/// Map u32 representing month in year to string containing Czech name of given month.
pub fn get_month_name(m: u32) -> String {
    match m {
         1 => String::from("Leden"),
         2 => String::from("Únor"),
         3 => String::from("Březen"),
         4 => String::from("Duben"),
         5 => String::from("Květen"),
         6 => String::from("Červen"),
         7 => String::from("Červenec"),
         8 => String::from("Srpen"),
         9 => String::from("Zaří"),
        10 => String::from("Říjen"),
        11 => String::from("Listopad"),
        12 => String::from("Prosinec"),
         _ => String::from(""),
    }
}