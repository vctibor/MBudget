use chrono::NaiveDate;
use chrono::prelude::*;
use model::Color;

//! Performs summarization calculations.

// TODO: Add colorization logic

/// Calculate total disposable amount for month.
pub fn total_disposable(
    daily_disposable: f64,
    days_in_month: u32) -> f64
{
    let days_in_month = days_in_month as f64;
    
    daily_disposable * days_in_month
}

/// Calculate remaining disposable amount for month.
pub fn amount_remaining(
    total_disposable: f64,
    amount_spent: f64) -> f64
{
    total_disposable - amount_spent
}

/// Calculate daily disposable amount.
/// This calculation actually makes sense only if it's for current month,
///   and there's larger than zero number of remaining days till end of month.
/// In all other cases we just return entire remaining amount.
pub fn real_daily_disposable(
    amount_remaining: f64,
    current_date: NaiveDate) -> (f64, Color)
{
    let year = current_date.year();
    let month = current_date.month();

    let last_day = 
        NaiveDate::from_ymd(year, month + 1, 1).pred().day();

    if last_day <= current_date.day() {
        return (amount_remaining, Color::Good)
    }

    let days_remaining = (last_day - current_date.day()) as f64;

    let real_daily_disposable = amount_remaining / days_remaining;

    (real_daily_disposable, Color::Good)
}

/// Calculate average amount spent per day in given month.
pub fn average_daily_spent(
    amount_spent: f64,
    current_date: NaiveDate) -> (f64, Color)
{
    let day = current_date.day() as f64;
    let average_daily_spent = amount_spent / day;

    (average_daily_spent, Color::Good)
}

/// "Saldo" is difference between what should have been spent by this time,
///   and what was actually spend.
/// It is calculated as difference between real_daily_disposable times day in month
///   and total amount spent.
pub fn saldo(
    real_daily_disposable: f64,
    amount_spent: f64,
    amount_remaining: f64,
    current_date: NaiveDate) -> (f64, Color)
{
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day() as f64;

    let last_day = 
        NaiveDate::from_ymd(year, month + 1, 1).pred().day();

    if last_day <= current_date.day() {
        return (amount_remaining, Color::Good);
    }

    let saldo = (real_daily_disposable * day) - amount_spent;

    (saldo, Color::Good)
}

/// Potential remain is amount remaining at the end of month,
///   if average spent per day won't change.
/// Thus, it's calculated as difference between 
pub fn potential_remaining(
    average_daily_spent: f64,
    amount_remaining: f64,
    current_date: NaiveDate) -> (f64, Color)
{
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();

    let last_day = 
        NaiveDate::from_ymd(year, month + 1, 1).pred().day();

    if last_day <= current_date.day() {
        return (amount_remaining, Color::Good);
    }

    let days_remaining = (last_day - day) as f64;

    let potential_remaining = amount_remaining - (average_daily_spent * days_remaining);

    (potential_remaining, Color::Good)
}