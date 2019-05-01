CREATE TABLE Categories (
    ID INTEGER NOT NULL,
    Name TEXT NOT NULL,
    PRIMARY KEY(ID)
);

CREATE TABLE Transactions (
    ID INTEGER NOT NULL,
    Date DATE NOT NULL,
    Category INTEGER REFERENCES Categories(ID), 
    Amount BIGINT NOT NULL,         -- milionths of Czech koruna
    Description TEXT,
    PRIMARY KEY(ID)
);

CREATE VIEW Days AS 
SELECT
    extract(day FROM date) AS Day,
    extract(month FROM date) AS Month,
    extract(year FROM date) AS Year,
    cast(sum(amount) as BIGINT) AS TotalSpent,
    count(amount) AS TransactionsCount
FROM Transactions
GROUP BY Day, Month, Year
ORDER BY Year, Month, Day;

GRANT ALL PRIVILEGES
ON ALL TABLES IN SCHEMA public TO malky;