//! Contains models for data access and UI.

use chrono::NaiveDate;

/// Represents single transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub date: NaiveDate,

    /// **TODO** : Option<Category>
    pub category: Option<i32>,
    
    pub amount: f64,
    pub description: Option<String>
}

/// Represents summarization of all transactions for given day.
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String
}

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

/// View model containing all summarization calculations.
#[derive(Serialize, Deserialize)]
pub struct InfoCalculationVM {

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

/// View model of category.
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryVM {
    pub id: i32,
    pub name: String,
    pub selected: bool
}

/// View model of transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionVM {
    pub id: i32,
    pub date: NaiveDate,
    pub categories: Vec<CategoryVM>,    
    pub amount: f64,
    pub description: Option<String>
}

/// This is the main view model. It represents data for given day and month.
///  It gets serialized into JSON and passed to Handlebars 'index' template,
///  which creates HTML page based on data in this model and template.
#[derive(Serialize, Deserialize)]
pub struct IndexVM {

    /// Name of the displayed month.
    pub month_name: String,

    /// Displayed year.
    pub year: i32,

    /// address of next month
    pub addr_nxt_month: String,
    
    /// address of previous month
    pub addr_prv_month: String,
    
    /// address of next day
    pub addr_nxt_day: String,
    
    /// address of previous day
    pub addr_prv_day: String,
    
    /// Summarization calculations.
    pub info: InfoCalculationVM,

    /// List of summarizations of transactions for each day.
    pub days: Vec<Day>,

    /// Current day in format "dd. MM."
    pub current_day: String,
    
    /// Name of current day in week.
    pub current_day_name: String,

    /// List of transactions for displayed day.
    pub transactions: Vec<TransactionVM>,

    // /// List of all transaction categories defined in system. 
    // pub categories: Vec<Category>
}