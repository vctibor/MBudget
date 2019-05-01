extern crate postgres;
extern crate chrono;
extern crate once_cell;
extern crate handlebars;
#[macro_use] extern crate serde;
#[macro_use] extern crate rouille;
#[macro_use] extern crate serde_json;
// #[macro_use] extern crate serde_derive;

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

// postgres://malky:malky@192.168.196.186:5432/mbudget

// https://doc.rust-lang.org/cargo/reference/manifest.html

const TEMPLATE_NAME: &str = "month";


const CONNECTION_STR: &str = "postgres://malky:malky@192.168.196.97:5432/mbudget";

// Amount of money allowed to spend each day.
// TODO: Move to DB and read on each HTTP request.
const DAILY_ALLOWANCE: f64 = 300.0;

static HBS: OnceCell<Handlebars> = OnceCell::INIT;

/// Get vector of days in given month and year.
///
/// Algorithm:
///  - get first day of next month
///  - get one day before - certain to be last day of given month
///  - iterate in simple integer for loop from 1 to last day of month
///  - build dates from numbers
fn get_month_days(year: u32, month: u32) -> Vec<NaiveDate> {

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


fn index_handler() -> Response {
    
    let now = now();
    
    let current_month = now.month();

    let current_year = now.year() as u32;
    
    //index_month_handler(current_year, current_month)
    index_month_handler(current_year, 4)
}

// TODO: show days without transactions
fn index_month_handler(year: u32, month: u32) -> Response {    

    let conn: Connection = Connection::connect(CONNECTION_STR, TlsMode::None).unwrap();   
    
    let expenses: HashMap<u32, DailyExpense> = read_month_transactions(&conn, year, month);

    let days = get_month_days(year, month);

    let mut model_days: Vec<Day> = Vec::new();

    let month_sum = get_month_sum(&conn, year, month);

    for day in &days {

        /*
        let color = if sum.total_spent <= DAILY_ALLOWANCE {
            Color::good
        } else {
            Color::bad
        };
        
        days.push(Day {
            day: sum.day,
            color: color,
            amount: sum.total_spent
        });
        */

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

    let model = IndexModel {
        month_name: "Duben".to_string(),
        year: 2019,

        total_disposable: 9300.0,
        day_disposable: 300.0,
        expenses_total: 5689.58,
        remaining_amount: 3610.42,

        real_day_disposable: 345.03,
        avg_daily_expenses: 155.34,
        saldo: 540.4,
        potential_remaining: 2276.36,

        real_day_disposable_color: Color::Good,
        avg_daily_expenses_color: Color::Bad,
        saldo_color: Color::Good,
        potential_remaining_color: Color::Bad,

        days: model_days,

        current_day: "4. Dubna".to_string(),
        current_day_name: "Čtvrtek".to_string()
    };

    let json_value: Value = json!(model);

    let handlebars = HBS.get().unwrap();
    
    let res = handlebars.render(TEMPLATE_NAME, &json_value).unwrap();
    
    rouille::Response::html(res)
}

fn now() -> NaiveDate {
    let d = Local::now().date();
    NaiveDate::from_ymd(d.year(), d.month(), d.day())
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

            
            (GET) (/{year: u32}/{month: u32}) => {
                index_month_handler(year, month)
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
