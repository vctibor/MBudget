/*
extern crate chrono;
extern crate postgres;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;

use chrono::NaiveDate;
use chrono::prelude::*;
use postgres::{Connection, TlsMode};

mod model;
use model::*;

mod date_utils;
use date_utils::*;

mod data_access;
use data_access::*;

mod calculations;

const DAILY_ALLOWANCE: f64 = 300.0;

const CONNECTION_STR: &str = "postgres://malky:malky@192.168.196.97:5432/mbudget";


fn main() {
    
    let conn: Connection = Connection::connect(CONNECTION_STR, TlsMode::None).unwrap();   

    let month: u32 = 3;

    let year: u32 = 2019;

    println!("Performing calculation for {}. month of year {}.",
        month, year);


    let days = get_month_days(year, month);

    let days_in_month = days.last().unwrap().day();

    let date = {

        let current_date = now();

        if current_date.year() == year as i32 && current_date.month() == month {
            current_date
        } else {
            NaiveDate::from_ymd(year as i32, month, days_in_month)
        }

    };

    println!("Calculation date: {:?}", date);

    println!("\n");
    
    let total_disposable = calculations::total_disposable(DAILY_ALLOWANCE, days_in_month);

    println!("Total disposable: {:.2}", total_disposable);

    let day_disposable = DAILY_ALLOWANCE;

    println!("Day disposable: {:.2}", day_disposable);

    let amount_spent = get_month_spent(&conn, year, month);

    println!("Amount spent: {:.2}", amount_spent);

    let amount_remaining = calculations::amount_remaining(total_disposable, amount_spent);

    println!("Amount remaining: {:.2}", amount_remaining);

    let real_daily_disposable = calculations::real_daily_disposable(
        amount_remaining, date);

    println!("Real daily disposable: {:.2}", real_daily_disposable);

    let average_daily_spent = calculations::average_daily_spent(
        amount_spent, date);

    println!("Average daily spent: {:.2}", average_daily_spent);

    let saldo = calculations::saldo(
        real_daily_disposable, amount_spent, amount_remaining, date);

    println!("Saldo: {:.2}", saldo);

    let potential_remaining = calculations::potential_remaining(
        average_daily_spent, amount_remaining, date);

    println!("Potential remaining: {:.2}", potential_remaining);
}
*/