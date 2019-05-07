//! Performs summarization calculations.

use chrono::NaiveDate;
use chrono::prelude::*;
use model::*;

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
    daily_disposable: f64,
    current_date: NaiveDate) -> (f64, Color)
{
    let year = current_date.year();
    let month = current_date.month();

    let last_day = 
        NaiveDate::from_ymd(year, month + 1, 1).pred().day();

    let real_daily_disposable = if last_day <= current_date.day() {
        amount_remaining
    } else {
        let days_remaining = (last_day - current_date.day()) as f64;

        amount_remaining / days_remaining
    };

    let color = if real_daily_disposable >= daily_disposable {
        Color::Good
    } else {
        Color::Bad
    };

    (real_daily_disposable, color)
}

/// Calculate average amount spent per day in given month.
pub fn average_daily_spent(
    amount_spent: f64,
    daily_disposable: f64,
    current_date: NaiveDate) -> (f64, Color)
{
    let day = current_date.day() as f64;
    let average_daily_spent = amount_spent / day;

    let color = if daily_disposable >= average_daily_spent {
        Color::Good
    } else {
        Color::Bad
    };

    (average_daily_spent, color)
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

    let potential_remaining = if last_day <= current_date.day() {
        amount_remaining
    } else {
        let days_remaining = (last_day - day) as f64;
        amount_remaining - (average_daily_spent * days_remaining)
    };

    let color = if potential_remaining >= 0.0 {
        Color::Good
    } else {
        Color::Bad
    };

    (potential_remaining, color)
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

    let saldo = if last_day <= current_date.day() {
        amount_remaining
    } else {
        (real_daily_disposable * day) - amount_spent
    };

    let color = if saldo >= 0.0 {
        Color::Good
    } else {
        Color::Bad
    };

    (saldo, color)
}

pub fn get_calculations(
    daily_disposable: f64,
    amount_spent: f64,
    date: NaiveDate) -> InfoCalculation {

    let last_day = 
        NaiveDate::from_ymd(date.year(), date.month() + 1, 1).pred().day();

    let total_disposable = total_disposable(daily_disposable, last_day);

    let amount_remaining = amount_remaining(total_disposable, amount_spent);

    let (real_daily_disposable, real_daily_disposable_color) = real_daily_disposable(
        amount_remaining, daily_disposable, date);

    let (average_daily_spent, average_daily_spent_color) = average_daily_spent(
        amount_spent, daily_disposable, date);

    let (saldo, saldo_color) = saldo(
        real_daily_disposable, amount_spent, amount_remaining, date);

    let (potential_remaining, potential_remaining_color) = potential_remaining(
        average_daily_spent, amount_remaining, date);

    InfoCalculation {
        total_disposable: format!("{:.2}", total_disposable),
        day_disposable: format!("{:.2}", daily_disposable),
        expenses_total: format!("{:.2}", amount_spent),
        remaining_amount: format!("{:.2}", amount_remaining),

        real_day_disposable: format!("{:.2}", real_daily_disposable),
        avg_daily_expenses: format!("{:.2}", average_daily_spent),
        saldo: format!("{:.2}", saldo),
        potential_remaining: format!("{:.2}", potential_remaining),

        real_day_disposable_color: real_daily_disposable_color,
        avg_daily_expenses_color: average_daily_spent_color,
        saldo_color: saldo_color,
        potential_remaining_color: potential_remaining_color
    }
}