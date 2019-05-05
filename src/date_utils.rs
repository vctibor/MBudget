use chrono::NaiveDate;
use chrono::prelude::*;

pub fn now() -> NaiveDate {
    let d = Local::now().date();
    NaiveDate::from_ymd(d.year(), d.month(), d.day())
}

/// Get vector of days in given month and year.
///
/// Algorithm:
///  - get first day of next month
///  - get one day before - certain to be last day of given month
///  - iterate in simple integer for loop from 1 to last day of month
///  - build dates from numbers
pub fn get_month_days(year: u32, month: u32) -> Vec<NaiveDate> {

    let year = year as i32;    

    let last_day = 
        NaiveDate::from_ymd(year, month + 1, 1).pred().day();

    let mut days: Vec<NaiveDate> = Vec::with_capacity(5);

    for day in 1..(last_day+1) {
        days.push(NaiveDate::from_ymd(year, month, day));
    }

    let days = days;

    days
}