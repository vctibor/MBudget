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
pub fn get_month_days(year: i32, month: u32) -> Vec<NaiveDate> {

    let last_day = 
        NaiveDate::from_ymd(year, month + 1, 1).pred().day();

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
pub fn get_month_name(m: u32) -> &'static str {
    match m {
         1 => "Leden",
         2 => "Únor",
         3 => "Březen",
         4 => "Duben",
         5 => "Květen",
         6 => "Červen",
         7 => "Červenec",
         8 => "Srpen",
         9 => "Zaří",
        10 => "Říjen",
        11 => "Listopad",
        12 => "Prosinec",
         _ => "",
    }
}
