# sql
Structured Query Language - aka CSV++.

## Overview
```SQL
// Create a new table with a custom primary key
CREATE TABLE groceries (id INTEGER PRIMARY KEY, name TEXT, quantity INTEGER);

// Create a new table if it doesn't exist already
CREATE TABLE IF NOT EXISTS groceries (id INTEGER PRIMARY KEY, name TEXT, quantity INTEGER);;

// Insert a value
INSERT INTO groceries VALUES (1, "Bananas", 4);

// Select a value
SELECT <names> FROM <table> WHERE <filter> ORDER BY <column>;
SELECT * FROM groceries WHERE quantity > 3 ORDER BY quantity;

// Aggregate functions
SELECT MAX(quantity) FROM <column>; // get max value
SELECT SUM(quantity) FROM <column>; // get total value
SELECT aisle, SUM(quantity) FROM <column>, GROUP BY aisle;  // group by key, then calculate max per thingy
```

## See Also
- [sqlite query planner](https://www.sqlite.org/optoverview.html)
- https://www.khanacademy.org/computing/computer-programming/sql/sql-basics
