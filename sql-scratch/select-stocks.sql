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
-- n = n-th page the query should fetch.
-- Extra arithmetic is needed to perform keyset-pagination. Bind `n` on the
--  client-side and query. Here `n` is ZERO-BASED index.
-- Keyset-pagination is choosen over limit-pagination in favor of performance
--  as the db will eventually run in client computer in production for offline-first
--  capability and privacy concerns.
-- Table only shows records that are expiring this week in each round-trip. Change the
--  interval offset to change the behavior.
--  e.g. '+7 day' --> '+21 day' for 3-weeks results.
    date_expiry >= CURRENT_DATE + interval '+7 day' * n
    AND
    date_expiry < CURRENT_DATE  + interval '+7 day' + interval '+7 day' * n
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
