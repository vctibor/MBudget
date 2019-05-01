extern crate postgres;
extern crate chrono;
extern crate once_cell;
extern crate handlebars;
#[macro_use] extern crate serde;
#[macro_use] extern crate rouille;
#[macro_use] extern crate serde_json;
// #[macro_use] extern crate serde_derive;

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
// TODO: Move to DB.
const DAILY_ALLOWANCE: f64 = 300.0;

static HBS: OnceCell<Handlebars> = OnceCell::INIT;


fn get_month_days(month: u32, year: u32) -> Vec<NaiveDate> {    

    /*
    let mut days = Vec::<NaiveDate>::new();    
    let mut dt = NaiveDate::from_ymd(year as i32, month, 01);

    loop {
        days.push(dt.clone());
        dt = dt.succ();
        if dt.month() != month {
            break;
        }
    }

    let days = days;
    return days;
    */


    let mut days = Vec::<NaiveDate>::new();    
    
    let mut dt = NaiveDate::from_ymd(year as i32, month, 01);

    let mut ix = 0;

    loop {
        
        days.insert(ix, dt.clone());
        
        ix = ix + 1;

        dt = dt.succ();
        
        if dt.month() != month {
            break;
        }
    }

    let days = days;
    return days;


}


fn index_handler() -> Response {
    
    let now = now();
    
    let current_month = now.month();

    let current_year = now.year() as u32;
    
    index_month_handler(current_year, current_month)
}

// TODO: show days without transactions
fn index_month_handler(year: u32, month: u32) -> Response {    

    let conn: Connection = Connection::connect(CONNECTION_STR, TlsMode::None).unwrap();
   
    let daily_sum = read_month_transactions(&conn, 3, year);

    let mut days = Vec::new();

    for sum in &daily_sum {

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

    }

    /*
    let days: Vec<Day> = vec![
        Day { day: 1, color: Color::bad, amount: 477.5 },
        Day { day: 2, color: Color::good, amount: 0.0 },
        Day { day: 3, color: Color::good, amount: 41.0 },
        Day { day: 4, color: Color::bad, amount: 558.0 },
        Day { day: 5, color: Color::good, amount: 133.52 },
        Day { day: 6, color: Color::bad, amount: 1200.0 },
        Day { day: 7, color: Color::bad, amount: 301.55 },
        Day { day: 8, color: Color::good, amount: 88.0 },
        Day { day: 9, color: Color::good, amount: 0.0 },
        Day { day:10, color: Color::good, amount: 15.5 }        
    ];
    */
    

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

        real_day_disposable_color: Color::good,
        avg_daily_expenses_color: Color::bad,
        saldo_color: Color::good,
        potential_remaining_color: Color::bad,

        days: days,

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

    // let conn: Connection = Connection::connect(CONNECTION_STR, TlsMode::None).unwrap();

    /*

    let now = now();

    println!("{:?}", now);

    let current_day = now.day();

    let current_month = now.month();

    let current_year = now.year() as u32;


    println!("Today is {}-th day of {}-th month.", current_day, current_month);


    let mut month_days: Vec<NaiveDate> = get_month_days(current_month, current_year);

    while let Some(date) = month_days.pop() {
        
        let d = date.day() as u32;
        let m = date.month() as u32;
        let y = date.year() as u32;

        let mut daily_trans: Vec<Transaction> = read_transactions(&conn, d, m, y);

        println!("{}", date);

        while let Some(tran) = daily_trans.pop() {
            println!("{} kč", tran.amount);
        }

        println!("\n");
    }


    let mut daily_trans: Vec<Transaction> = read_transaction(&conn, 4, 4, 2019);


    while let Some(tran) = daily_trans.pop() {
        
        println!("{:?}", tran);
    }
    */


    
    let address = ("192.168.1.2").to_owned();

    let port = ("9000").to_owned();

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

    let addr = address + ":" + &port.to_string();

    println!("Started server on {}", addr);

    rouille::start_server(addr, move |request| {
    
        let response = rouille::match_assets(&request, &wwwroot_location);

        if response.is_success() {
            return response;
        }

        router!(request,

            
            (GET) (/) => { index_handler() },

            /*
            (GET) (/{year: u32}/{month: u32}) => {
                index_month_handler(year, month)
            },

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


    /*
    let query = "select id, name from categories";

    let query_result = &conn.query(query, &[]).expect("Query failed.");

    for row in query_result {

        let cat = Category {
            id: row.get(0),
            name: row.get(1)
        };

        println!("{:?}", cat);
    }
    
    println!("\n\n");

    let query = "select id, date, category, amount, description from transactions";

    let query_result = &conn.query(query, &[]).expect("Query failed.");

    for row in query_result {

        let tran = Transaction {
            id: row.get(0),
            date: row.get(1),
            category: row.get(2),
            amount: row.get(3),
            description: row.get(4)
        };

        println!("{:?}", tran);
    }

    
    println!("\n\n");


    let query = "select * from days";

    let query_result = &conn.query(query, &[]).expect("Query failed.");

    for row in query_result {

        let day: f64 = row.get(0);
        let month: f64 = row.get(1);
        let total: i64 = row.get(2);
        let transactions: i64 = row.get(3);

        println!("{} {} {} {}", day, month, total, transactions);
    }
    */


}
