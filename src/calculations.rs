/// Performs summarization calculations.

/// Calculate total disposable amount for month.
/// Calculated as:
/// daily_disposable * days_in_month
fn total_disposable(daily_disposable: f64, days_in_month: u32) -> f64 {
    let days_in_month = days_in_month as f64;
    daily_disposable * days_in_month
}

//fn amount_remaining(total_disposable: f64, )