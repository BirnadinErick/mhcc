-- For this to work, FTS should be bootstrapped beforehand. SQL command
--  for that is found in db-seed.sql
SELECT 
    stock_id, 
    stocks.name as stock_name, 
    uprice, 
    quantity, 
    date_expiry,
    dispensers.name as dispensers_name
FROM stocks
LEFT JOIN dispensers
    ON stocks.dispenser_id = dispensers.dispenser_id
-- Bind the query instead of 'query'.
-- `plainto_tsquery` is choosen as names of the stocks will be literals
WHERE stocks.search_tokens @@ plainto_tsquery('query')
ORDER BY date_expiry ASC, stock_id ASC;