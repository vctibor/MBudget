select
    extract(month from date) as month,
    extract(year from date) as year,
	300000000 as original_daily_disposable,  -- TODO: move to database
	cast(sum(amount) as bigint) as total_spent,
	
	extract(day from (date_trunc('month',
		make_date((extract(year from date))::int, (extract(month from date))::int, 1))
			+ interval '1 month' - interval '1 day')::date) as last_day
from transactions
group by month, year
order by year, month