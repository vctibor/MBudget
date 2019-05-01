//use chrono::NaiveDate;
//use serde_json::Value;

// data model

/*
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
*/



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
pub struct IndexModel {

    pub month_name: String,
    pub year: u32,
    
    pub total_disposable: f64,
    pub day_disposable: f64,
    pub expenses_total: f64,
    pub remaining_amount: f64,

    pub real_day_disposable: f64,
    pub avg_daily_expenses: f64,
    pub saldo: f64,
    pub potential_remaining: f64,

    pub real_day_disposable_color: Color,
    pub avg_daily_expenses_color: Color,
    pub saldo_color: Color,
    pub potential_remaining_color: Color,

    pub days: Vec<Day>,

    pub current_day: String,
    pub current_day_name: String
}