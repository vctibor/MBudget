use postgres::Connection;
use chrono::NaiveDate;
//use model::Transaction;

#[derive(Debug)]
pub struct Transaction {
    pub id: i32,
    pub date: NaiveDate,
    pub category: Option<i32>,
    pub amount: f64,
    pub description: Option<String>
}

#[derive(Debug)]
pub struct DailyExpense {
    pub day: u32,
    pub month: u32,
    pub year: u32,
    pub total_spent: f64,
    pub trans_count: i64
}

const SCALE: f64 = 1_000_000.0;

pub fn read_transactions(conn: &Connection, day: u32, month: u32, year: u32) -> Vec<Transaction> {

    /*
    This isn't perfect - we should use query interpolation
        instead of string concatenation for dynamic parameters
        because of sql injection; however, that doesn't work for some reason.
    I'm leaving it this way for now (a.k.a. forever) because it's private
        application, where security is obtained through different means
        (i.e. not being exposed to Internet).
    */

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

pub fn read_month_transactions(conn: &Connection, month: u32, year: u32) -> Vec<DailyExpense> {
    

    let month = month as f64;
    let year = year as f64;

    /*
    let query = format!(
        "select * from days where month = {m} and year = {}",
        y = year, m = month, d = day);
    */

    let query = "select * from days where month = $1 and year = $2";

    let query_result: &postgres::rows::Rows =
        &conn.query(&query, &[&month, &year]).expect("Query failed.");

    let mut vec = Vec::new();

    for row in query_result {

        let d: f64 = row.get(0);
        let m: f64 = row.get(1);
        let y: f64 = row.get(2);

        
        let amount: i64 = row.get(3);

        let amount: f64 = amount as f64;

        let amount = amount / SCALE;
        
        let exp = DailyExpense {
            day: d as u32,
            month: m as u32,
            year: y as u32,
            total_spent: amount,
            trans_count: row.get(4)
        };
        
        vec.push(exp);
    }

    let vec = vec;

    vec
}