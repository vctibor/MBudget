Theory
======

Basic idea is decoupling between income and expenses. There is concept of *disposable amount* which is fixed independently of income.

We assume two accounts: regular one and saving one. At the beginning of accoutning period *total disposale amount* for given period is transfered from saving account to regular one, while remainder from previous accounting period is left on regular account as a cushion.

When salary is received, we pay all expenses not counted into disposable amount, and all that remains is transfered into saving account.

(For more info see *Firefly III* documentation.)

Accounting period is from first day of month, to last day of month, independently of income.

Disposable amount doesn't include expenses which are regular and invariable, such as:

- rent

- power

- internet connection

- savings

Disposable amount includes food, because there is huge variability of possible expenses. All other expenses not mentioned before are counted into disposable amount.

Disposable amount doesn't include regular and invariable incomes, in other words salary. Disposable amount includes irregular and variable incomes.

Summarization calculations
--------------------------

Summarization calculations are set of variables, calculated for each month, based on data from given month. Their purpose is:

- Offer easy insight into financial situation in current month (do I spend more than I should? can I afford larger expense? how many days do I have to starve so I can buy *x*?)

- Provide overview of financial behaviour in the past.

We define *total_disposable_amount* for given accouting period. Based on it we calculate *original_daily_disposable* such that:

    original_daily_disposable = total_disposable_amount / (days in month)

That means if I spend exactly *original_daily_disposable* each day, I'll have 0,- left at the end of month.

For each day we calculate *sum_of_daily_expenses* which is just sum of all transaction amounts for given day.

For given day in month *cd* and remaining days in month *rd* we can calculate following variables:

*total_spent* as

    total_spent = sum(sum_of_daily_expenses)

*remaining_disposable* as

    remaining_disposable = total_disposable_amount - total_spent

*average_daily_spent* as

    average_daily_spent = total_disposable_amount / rd


Analytics
---------

Page on address */analytics* will contain plots for analysis of longer term spending habits.

Implemented in [plotly](https://plot.ly/javascript/), server will serve basic page containing inputs and divs for plotting graphs; javascript will call AJAX methods that serve JSON values for given inputs, and plot them.

It will allow to set lower and upper bound for period and granularity (daily, monthly, yearly).

Plotted values are:

- Total spent for each granularity unit (each day for daily granularity, etc.).

- Total spent per category in given period, ordered from highest to lowest.

TODO
----

- Delete record on receiving entry containing valid ID and empty or zero amount

- Move summarization calculation into database (view or stored procedure or function)

- Move original daily allowance into database in format [*amount*, *validSince*], when reading use either:

    - entry with lowest *validSince* if reading for date which is lower than lowest *validSince*

    - entry with highest *validSince* if reading for date higher than highest *validSince*

    - entry with lower *validSince* if reading for date which falls between two entries

- Analytics

- General error handling

- Logging