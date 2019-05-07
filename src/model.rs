use chrono::NaiveDate;
//use serde_json::Value;

// data model

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

#[derive(Serialize, Deserialize)]
struct Month {
    month: u32,
    year: u32,
    name: String,
    weeks: Vec<Day>
}

#[derive(Debug)]
struct Category {
    id: i32,
    name: String
}



// view models

#[derive(Serialize, Deserialize)]
pub enum Color {
    Good,
    Bad
}

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub day: u32,
    pub color: Color,
    pub amount: f64
}

#[derive(Serialize, Deserialize)]
pub struct InfoCalculation {

    pub total_disposable: String,
    pub day_disposable: String,
    pub expenses_total: String,
    pub remaining_amount: String,

    pub real_day_disposable: String,
    pub avg_daily_expenses: String,
    pub saldo: String,
    pub potential_remaining: String,

    pub real_day_disposable_color: Color,
    pub avg_daily_expenses_color: Color,
    pub saldo_color: Color,
    pub potential_remaining_color: Color
}

#[derive(Serialize, Deserialize)]
pub struct IndexModel {

    pub month_name: &'static str,
    pub year: i32,
    
    pub info: InfoCalculation,

    pub days: Vec<Day>,

    pub current_day: String,
    pub current_day_name: String
}