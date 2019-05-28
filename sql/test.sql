select
	month,
	year,
	original_daily_disposable,
	total_disposable,
	total_spent,
	(total_disposable - total_spent) as total_remaining,
	(total_spent / last_day) as average_spent,
	((total_disposable - total_spent) / last_day) as real_daily_disposable
from (
	select
		*,
		(original_daily_disposable * last_day) as total_disposable
	from (
		select
			extract(month from date) as month,
			extract(year from date) as year,
			extract(day from (date_trunc('month',
			make_date((extract(year from date))::int, (extract(month from date))::int, 1))
				+ interval '1 month' - interval '1 day')::date) as last_day,
			cast(300000000 as double precision) as original_daily_disposable,  -- TODO: move to database
			cast(sum(amount) as double precision) as total_spent
		from transactions
		group by month, year
	) n1
) n2