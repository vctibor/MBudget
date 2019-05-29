extern crate postgres;
extern crate chrono;
extern crate once_cell;
extern crate handlebars;
extern crate toml;
extern crate clap;
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
use rouille::{Response, Request};
use serde_json::Value;
use std::io::Read;
use clap::{App, Arg};
use std::fs::File;

use data_access::*;
use date_utils::*;
use model::*;
use calculations::*;

/// String used to register template in Handlebars templating engine.
const INDEX_TEMPLATE_NAME: &str = "INDEX";
const ANALYTICS_TEMPLATE_NAME: &str = "ANALYTICS";


/// Amount of money allowed to spend each day.
/// **TODO: Move to DB and read on each HTTP request.**
const DAILY_ALLOWANCE: f64 = 300.0;

static HBS: OnceCell<Handlebars> = OnceCell::INIT;

static CATEGORIES: OnceCell<Vec<Category>> = OnceCell::INIT;

fn index_handler(conn_str: &str) -> Response {
    index_month_handler(now().year(), now().month(), now().day(), conn_str)
}

fn index_month_handler(year: i32, month: u32, day: u32, conn_str: &str) -> Response {  

    let conn: Connection = Connection::connect(conn_str, TlsMode::None)
        .expect("Failed to connect to database.");   
    
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
    
    let categories = CATEGORIES.get()
        .expect("Failed to read OnceCell containing categories.");
    
    // Constructs Transactions viewmodel. Probably can be done in more idiomatic and shorter way.
    let transactions_view: Vec<TransactionVM> = {

        let count = day_transactions.len();

        let mut transactions_view = Vec::with_capacity(count);

        for i in 0..count {

            let t = &day_transactions[i];
            
            let cats = {
                let cat_count = categories.len();
                let mut cats = Vec::with_capacity(cat_count);
                for j in 0..cat_count {

                    let selected_cat =
                        t.category.is_some() && categories[j].id == t.category.unwrap();

                    cats.push(CategoryVM {
                        id: categories[j].id,
                        name: categories[j].name.clone(),
                        selected: selected_cat
                    });
                }
                cats
            };

            transactions_view.push(TransactionVM {
                id: t.id.unwrap(),
                date: t.date,
                categories: cats,    
                amount: t.amount,
                description: t.description.clone()
            });
        }

        transactions_view
    };

    let categories_view = {
        let mut c = Vec::with_capacity(categories.len());

        for i in 0..categories.len() {
            c.push(CategoryVM {
                id: categories[i].id,
                name: categories[i].name.clone(),
                selected: false
            });
        }

        c
    };

    let model = IndexVM {
        day: day,
        month: month,
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
        transactions: transactions_view,
        categories: categories_view
    };

    let json_value: Value = json!(model);

    let handlebars = HBS.get()
        .expect("Failed to read OnceCell containing handlebars templates.");
    
    let res = handlebars.render(INDEX_TEMPLATE_NAME, &json_value)
        .expect("Failed to render Index template.");
    
    rouille::Response::html(res)
}

fn write_event_handler(year: i32, month: u32, day: u32, request: &Request, conn_str: &str) {

    let conn: Connection = Connection::connect(conn_str, TlsMode::None)
        .expect("Failed to connect to database.");   
    
    #[derive(Debug, Serialize, Deserialize)]
    struct InputTransaction {
        id: Option<i32>,
        amount: Option<f64>,
        category: Option<i32>,
        description: String
    };

    let mut data = "".to_string();

    request.data().unwrap().read_to_string(&mut data).unwrap();

    let records: Vec<InputTransaction> =
        serde_json::from_str(&data).unwrap();
    
    let mut updates: Vec<Transaction> =
        Vec::with_capacity(records.len());

    let date = NaiveDate::from_ymd(year, month, day);

    for record in records {

        // If amount is none, don't write into DB.
        // TODO: if ID is Some, delete tran

        if record.amount.is_none() && record.id.is_none() {
            continue;
        }

        let transaction = Transaction {
            id: record.id,
            date: date.clone(),
            category: record.category,            
            amount: record.amount.unwrap(),
            description: Some(record.description)
        };
        
        updates.push(transaction);
    }

    upsert_transactions(&conn, updates);
}

/// Serve analytics web page.
fn analytics_handler() -> Response {

    let model = ();

    let json_value: Value = json!(model);

    let handlebars = HBS.get()
        .expect("Failed to read OnceCell containing handlebars templates.");
    
    let res = handlebars.render(ANALYTICS_TEMPLATE_NAME, &json_value)
        .expect("Failed to render Index template.");
    
    rouille::Response::html(res)
}

/// Serve JSON values to analytics page.
fn analytics_data_handler(conn_str: &str) -> Response {

    let conn: Connection = Connection::connect(conn_str, TlsMode::None)
        .expect("Failed to connect to database.");

    let result = get_daily_transactions(&conn);

    let data = AnalyticsData {
        daily_expenses: result
    };

    Response::json(&data)
}

fn main() {

    let options = App::new("MBudget")
        .arg(Arg::with_name("file")
            .index(1)
            .help("TOML config")
            .required(true)
            .takes_value(true))
        .get_matches();

    let filename = options.value_of("file")
        .expect("Path to configuration file is required parameter. Aborting.");
    
    let mut file = File::open(filename)
        .expect("Couldn't open configuration file. Aborting.");

    let mut contents = String::new();        

    file.read_to_string(&mut contents)
        .expect("Couldn't read configuration file. Aborting.");

    let config: Config = toml::from_str(&contents)
        .expect("Couldn't parse configuration file. Make sure it is valid TOML. Aborting.");

    let address = config.address.to_owned();
    let port = config.port.to_owned();
    let connection_string = config.conn_string.to_owned();
    let templates_location = config.templates.to_owned();
    let wwwroot_location = config.wwwroot.to_owned();

    let handlebars = {

        let mut handlebars = Handlebars::new();

        let index = templates_location.clone() + "//index.hbs";

        handlebars.register_template_file(INDEX_TEMPLATE_NAME, index)
            .expect("Failed to register index template to Handlebars registry. Aborting.");

        let analytics = templates_location + "//analytics.hbs";

        handlebars.register_template_file(ANALYTICS_TEMPLATE_NAME, analytics)
            .expect("Failed to register analytics template to Handlebars registry. Aborting.");

        // Panic on unknown variables in template
        handlebars.set_strict_mode(true);

        handlebars
    };

    HBS.set(handlebars)
        .expect("Couldn't set Handlebars registry to OnceCell. Aborting.");

    let categories: Vec<Category> = {
        let conn = Connection::connect(connection_string.clone(), TlsMode::None)
            .expect("Failed to connect to database.");

        get_categories(&conn)
    };

    CATEGORIES.set(categories)
        .expect("Couldn't set categories vector to OnceCell. Aborting.");
    

    let addr = address + ":" + &port.to_string();

    rouille::start_server(addr, move |request| {
    
        let response = rouille::match_assets(&request, &wwwroot_location);

        if response.is_success() {
            return response;
        }

        router!(request,

            (GET) (/) => { index_handler(&connection_string) },
            
            (GET) (/{year: i32}/{month: u32}/{day: u32}) => {
                index_month_handler(year, month, day, &connection_string)
            },

            (POST) (/write-event/{year: i32}/{month: u32}/{day: u32}) => {
                write_event_handler(year, month, day, &request, &connection_string);                
                rouille::Response::empty_204()
            },

            (GET) (/analytics) => { analytics_handler() },

            (GET) (/analytics/data) => {
                analytics_data_handler(&connection_string)
            },

            _ => rouille::Response::empty_404()    
        )
    });
}