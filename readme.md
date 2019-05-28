# MBudget

Web based budgeting application.

## Features

For detailed description see *theory.md* or *theory-english.md* files.

## TODO

- Delete record on receiving entry containing valid ID and empty or zero amount

- Move summarization calculation into database (view or stored procedure or function)

- Move original daily allowance into database in format [*amount*, *validSince*], when reading use either:

    - entry with lowest *validSince* if reading for date which is lower than lowest *validSince*

    - entry with highest *validSince* if reading for date higher than highest *validSince*

    - entry with lower *validSince* if reading for date which falls between two entries

- Analytics

- General error handling

- Logging

## Technical details

Written in Rust, using Postgres database.

Licensed under terms of GPLv3 license.