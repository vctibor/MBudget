extern crate postgres;
extern crate chrono;
extern crate once_cell;
extern crate handlebars;
#[macro_use] extern crate serde;
#[macro_use] extern crate rouille;
#[macro_use] extern crate serde_json;

use std::collections::HashMap;
use postgres::{Connection, TlsMode};
use chrono::NaiveDate;
use chrono::prelude::*;
use once_cell::sync::OnceCell;
use handlebars::Handlebars;
use rouille::Response;
use serde_json::Value;

mod model;
use model::*;

mod data_access;
use data_access::*;

mod date_utils;
use date_utils::*;

mod calculations;
use calculations::*;

const TEMPLATE_NAME: &str = "month";

const CONNECTION_STR: &str = "postgres://malky:malky@192.168.196.97:5432/mbudget";

// Amount of money allowed to spend each day.
// TODO: Move to DB and read on each HTTP request.
const DAILY_ALLOWANCE: f64 = 300.0;

static HBS: OnceCell<Handlebars> = OnceCell::INIT;

fn index_handler() -> Response {
    index_month_handler(now().year(), now().month(), now().day())
}

fn index_month_handler(year: i32, month: u32, day: u32) -> Response {  

    let conn: Connection = Connection::connect(CONNECTION_STR, TlsMode::None).unwrap();   
    
    let expenses: HashMap<u32, DailyExpense> = read_month_transactions(&conn, year, month);

    let days = get_month_days(year, month);

    let mut model_days: Vec<Day> = Vec::new();

    for day in &days {

        let day_in_month: u32 = day.day();

        let day_expense = expenses.get(&day_in_month);

        let mut color: Color = Color::Good;
        let mut amount: f64 = 0.0;

        if let Some(expense) = day_expense {

            color = if expense.total_spent <= DAILY_ALLOWANCE {
                Color::Good
            } else {
                Color::Bad
            };

            amount = expense.total_spent;
        } 

        model_days.push(Day {
            day: day_in_month,
            color: color,
            amount: amount
        });
    }

    
    
    let date = NaiveDate::from_ymd(year, month, day);

    let month_name = get_month_name(month);

    let day_name = get_weekday_name(date.weekday());

    //let daily_disposable = DAILY_ALLOWANCE;

    let amount_spent = get_month_spent(&conn, year, month);

    //et day_transactions = read_day_transactions(&conn, year, month, day);


    // If we are handling current month, we perform calculations for current date,
    //  otherwise calculate values for last day of given month.
    let calculation_date = if now().year() == year && now().month() == month {
        now()
    } else {
        let last_day = days.last().unwrap().day();
        NaiveDate::from_ymd(year, month, last_day)
    };

    let info = get_calculations(DAILY_ALLOWANCE, amount_spent, calculation_date);

    let model = IndexModel {
        month_name: month_name,
        year: year,
        info: info,
        days: model_days,
        current_day: format!("{}. {}", day, month_name),
        current_day_name: day_name
    };

    let json_value: Value = json!(model);

    let handlebars = HBS.get().unwrap();
    
    let res = handlebars.render(TEMPLATE_NAME, &json_value).unwrap();
    
    rouille::Response::html(res)
}

fn main() {


    
    //let address = ("192.168.1.2").to_owned();

    //let port = ("9000").to_owned();

    let wwwroot_location = ("./static").to_owned();

    let templates_location = ("./templates").to_owned();

    let handlebars = {

        let mut handlebars = Handlebars::new();

        let index = templates_location + "//index.hbs";

        handlebars.register_template_file(TEMPLATE_NAME, index)
            .expect("Failed to register template to Handlebars registry. Aborting.");

        handlebars
    };

    HBS.set(handlebars)
        .expect("Couldn't set Handlebars registry to OnceCell, it was already used. Aborting.");


    // Start server

    //let addr = address + ":" + &port.to_string();

    let addr = "0.0.0.0:9000";

    println!("Started server on {}", addr);

    rouille::start_server(addr, move |request| {
    
        let response = rouille::match_assets(&request, &wwwroot_location);

        if response.is_success() {
            return response;
        }

        router!(request,

            
            (GET) (/) => { index_handler() },

            
            (GET) (/{year: i32}/{month: u32}/{day: u32}) => {
                index_month_handler(year, month, day)
            },

            /*
            (GET) (/read-month/{year: u32}/{month: u32}) => {
                read_month_handler(year, month)
            },

            (POST) (/write-event/{year: u32}/{month: u32}/{day: u32}) => {
                write_event_handler(year, month, day, &request);                
                rouille::Response::empty_204()
            },
            */

            _ => rouille::Response::empty_404()
            
        )
    });

}
