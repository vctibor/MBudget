//! Abstraction layer for reading and writing data to/from PostgresSQL.

use std::collections::HashMap;
use postgres::Connection;
use model::*;
//use chrono::NaiveDate;
use chrono::prelude::*;

// TODO: use SERIAL pgsql type on id columns
// upsert modified records

const SCALE: f64 = 1_000_000.0;

///Get list of all unique transactions in given day.
///This isn't perfect - we should use query interpolation
///    instead of string concatenation for dynamic parameters
///    because of sql injection; however, that doesn't work for some reason.
///I'm leaving it this way for now (a.k.a. forever) because it's private
///    application, where security is obtained through different means
///    (i.e. not being exposed to Internet).
pub fn read_day_transactions(conn: &Connection, year: i32, month: u32, day: u32)
    -> Vec<Transaction>
{
    let query = format!(
        "select * from transactions where date = '{y}-{m}-{d}'",
        y = year, m = month, d = day);

    let query_result: &postgres::rows::Rows =
        &conn.query(&query, &[]).expect("Query failed.");

    let mut vec = Vec::new();

    for row in query_result {

        let amount: i64 = row.get(3);

        let amount: f64 = amount as f64;

        let amount = amount / SCALE;

        let tran = Transaction {
            id: row.get(0),
            date: row.get(1),
            category: row.get(2),
            amount: amount,
            description: row.get(4)
        };

        vec.push(tran);
    }

    let vec = vec;

    vec
}

/// Gets list of sums of total amount spent per each day in given month.
pub fn read_month_transactions(conn: &Connection, year: i32, month: u32)
    -> HashMap<u32, DailyExpense>
{    
    let month = month as f64;
    let year = year as f64;

    let query = "select * from days where month = $1 and year = $2";

    let query_result: &postgres::rows::Rows =
        &conn.query(&query, &[&month, &year]).expect("Query failed.");

    let mut map: HashMap<u32, DailyExpense> = HashMap::with_capacity(query_result.len());

    for row in query_result {

        let day: f64 = row.get(0);
        let day = day as u32;

        let month: f64 = row.get(1);
        let month = month as u32;

        let year: f64 = row.get(2);
        let year = year as u32;

        let amount: i64 = row.get(3);
        let amount = (amount as f64) / SCALE;
        
        let exp = DailyExpense {
            day: day,
            month: month,
            year: year,
            total_spent: amount,
            trans_count: row.get(4)
        };
        
        map.insert(day, exp);
    }

    map
}

/// Gets total sum of amount spent for given month.
/// Note we have to handle december specifically!
pub fn get_month_spent(conn: &Connection, year: i32, month: u32) -> f64 {

    let next_month = if month == 12 { 1 } else { month + 1 };
    let start = format!("{}-{}-01", year, month);
    let end = format!("{}-{}-01", year, next_month);

    let query = format!(
        "select cast(sum(amount) as bigint) from transactions
        where date >= '{}' and date < '{}'",
        start, end);

    let query_result: &postgres::rows::Rows =
        &conn.query(&query, &[]).expect("Query failed.");

    let result: Option<i64> = query_result.get(0).get(0);

    match result {
        Some(val) => { (val as f64) / SCALE }

        None => { 0.0 }
    }
}

/// Obtain list of all categories.
/// Returns empty list in case of failure (perhaps better return Option).
pub fn get_categories(conn: &Connection) -> Vec<Category> {

    let query = "select id, name from categories";

    let query_result = &conn.query(&query, &[]); //.expect("Query failed.");

    match query_result {
        Ok(rows) => {
            let mut vec = Vec::new();

            for row in rows {

                let cat = Category {
                    id: row.get(0),
                    name: row.get(1)
                };

                vec.push(cat);
            }

            let vec = vec;

            vec
        },

        Err(e) => {
            println!("Error querying categories: {:?}", e);
            Vec::new()
        }
    }
}

pub fn upsert_transactions(conn: &Connection, transactions: Vec<Transaction>)
{
    for transaction in transactions {

        let amount = (transaction.amount * SCALE) as i64;

        let descr = match transaction.description.clone() {
            Some(s) => s,
            None => "".to_string()
        };

        // insert
        if transaction.id.is_none() {

            let mut query = String::from("insert into transactions(date, amount, description");

            if transaction.category.is_some() {
                query.push_str(", category");
            }

            query.push_str(") values ");

            let val = format!("('{y}-{m}-{d}', {amount}, '{description}'",
                y = transaction.date.year(),
                m = transaction.date.month(),
                d = transaction.date.day(),
                amount = amount,
                description = descr);

            query.push_str(&val);

            if let Some(c) = transaction.category {
                query.push_str(&format!(", {}", c));
            }

            query.push_str(")");

            println!("{}", &query);

            conn.execute(&query, &[])
                .expect("Failed to execute insert transactions query.");
        }

        // update
        if transaction.id.is_some() {
            let mut query = String::from("update transactions set ");

            query.push_str(&format!("amount = {}, description = '{}'", amount, descr));

            if let Some(c) = transaction.category {
                query.push_str(&format!(", category = {}", c));
            }

            query.push_str(&format!(" where id = {}", transaction.id.unwrap()));

            println!("{}", &query);

            conn.execute(&query, &[])
                .expect("Failed to execute update transactions query.");
        }
    }
}