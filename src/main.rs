extern crate postgres;
extern crate chrono;
extern crate once_cell;  // TODO: try to replace OnceCell with lazy_static
extern crate handlebars;
#[macro_use] extern crate serde;
#[macro_use] extern crate rouille;
#[macro_use] extern crate serde_json;

mod model;
mod data_access;
mod date_utils;
mod calculations;

use std::collections::HashMap;
use postgres::{Connection, TlsMode};
use chrono::NaiveDate;
use chrono::prelude::*;
use once_cell::sync::OnceCell;
use handlebars::Handlebars;
use rouille::Response;
use serde_json::Value;

use data_access::*;
use date_utils::*;
use model::*;
use calculations::*;

/*
TODO:
DONE, KINDA - query DB for list of categories (new func in data_access)
- change day transactions to be inputs (text input, select, text input)
- populate select options with categories
- add unique ID to each row of day transactions
- add writing handler for enter key
*/

/// String used to register template in Handlebars templating engine.
const TEMPLATE_NAME: &str = "index_template";

/// Database connection string.
/// **TODO: Move into args/configuration.**
const CONNECTION_STR: &str = "postgres://malky:malky@192.168.196.97:5432/mbudget";

/// Amount of money allowed to spend each day.
/// **TODO: Move to DB and read on each HTTP request.**
const DAILY_ALLOWANCE: f64 = 300.0;

static HBS: OnceCell<Handlebars> = OnceCell::INIT;

//static CATEGORIES: OnceCell<Vec<Category>> = OnceCell::INIT;

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


    let amount_spent = get_month_spent(&conn, year, month);

    let day_transactions = read_day_transactions(&conn, year, month, day);


    // If we are handling current month, we perform calculations for current date,
    //  otherwise calculate values for last day of given month.
    let calculation_date = if now().year() == year && now().month() == month {
        now()
    } else {
        let last_day = days.last().unwrap().day();
        NaiveDate::from_ymd(year, month, last_day)
    };

    
    let addr_nxt_month = {
        let nxt_month = next_month(date);
        format!("/{}/{}/{}", nxt_month.year(), nxt_month.month(), nxt_month.day())
    };
    
    let addr_prv_month = {
        let prv_month = prev_month(date);
        format!("/{}/{}/{}", prv_month.year(), prv_month.month(), prv_month.day())
    };
    
    let addr_nxt_day = {
        let nxt_day = date.succ();
        format!("/{}/{}/{}", nxt_day.year(), nxt_day.month(), nxt_day.day())
    };
    
    let addr_prv_day = {
        let prv_day = date.pred();
        format!("/{}/{}/{}", prv_day.year(), prv_day.month(), prv_day.day())
    };


    let info = get_calculations(DAILY_ALLOWANCE, amount_spent, calculation_date);

    /*
    let cats: Vec<Category> = CATEGORIES.get()
        .expect("Failed to read OnceCell containing categories.");
    */

    // TODO: don't call on every request
    let cats = get_categories(&conn);


    let model = IndexModel {
        month_name: month_name.clone(),
        year: year,
        addr_nxt_month: addr_nxt_month,
        addr_prv_month: addr_prv_month,
        addr_nxt_day: addr_nxt_day,
        addr_prv_day: addr_prv_day,
        info: info,
        days: model_days,
        current_day: format!("{}. {}", day, month_name),
        current_day_name: day_name,
        transactions: day_transactions,
        categories: cats
    };

    let json_value: Value = json!(model);

    let handlebars = HBS.get()
        .expect("Failed to read OnceCell containing handlebars templates.");
    
    let res = handlebars.render(TEMPLATE_NAME, &json_value)
        .expect("Failed to render Index template.");
    
    rouille::Response::html(res)
}

fn main() {

    // TODO: Configuration
    let wwwroot_location = ("./static").to_owned();

    // TODO: Configuration
    let templates_location = ("./templates").to_owned();

    let handlebars = {

        let mut handlebars = Handlebars::new();

        let index = templates_location + "//index.hbs";

        handlebars.register_template_file(TEMPLATE_NAME, index)
            .expect("Failed to register template to Handlebars registry. Aborting.");

        handlebars
    };

    HBS.set(handlebars)
        .expect("Couldn't set Handlebars registry to OnceCell. Aborting.");

    /*
    let categories: Vec<Category> = {
        let conn = Connection::connect(CONNECTION_STR, TlsMode::None)
            .expect("Failed to connect to database.");

        get_categories(&conn)
    };

    CATEGORIES.set(categories)
        .expect("Couldn't set categories vector to OnceCell. Aborting.");
    */


    // TODO: Configuration
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
