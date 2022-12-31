SELECT 
    stock_id, 
-- Alias is needed for the sqlx to deserialize the SQL result to named-struct
-- field. This is due to the fact that rust struct fiels names are snake_case;
    stocks.name as stock_name, 
    dispensers.name as dispensers_name,
    uprice, 
    quantity, 
    date_expiry,
    staff_name
FROM stocks
-- Here `LEFT JOIN` is choosen to avoid losing
--  any stock data that might lack other relational data due to 
--  human error during data entry;
LEFT JOIN staffs 
    ON stocks.staff_stocked = staffs.staff_id
LEFT JOIN dispensers
    ON stocks.dispenser_id = dispensers.dispenser_id
WHERE 
    stocks.name LIKE 'dr%'
-- Query is sorted based in two conditionals as only one (based of date_expiry) produced
--  non-idempotant fetch.
-- Sort using stock_id is enow as stock_id is unique per-day AND integer,
--  only one permutation is available for a particular date.
ORDER BY date_expiry ASC, stock_id ASC
-- Entire record set for a given range is fetched as oppose to chunck-based fetching. And,
--  `FETCH` cluase is ommitted delibratively as changing FETCH-size and the range simultaneously
--  produced indeterministic results. This should be fixed later on.
-- FETCH FIRST 100 ROWS ONLY
;
