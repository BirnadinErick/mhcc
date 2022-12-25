-- SEED DATABASE FOR DEVELOPEMENT
-- ==============================
-- data fiels are located inside db-seed.zip in the root directory
-- all the data are randomly generated using python faker library
-- seed-file may (not) synced with the current schema!

-- BEGIN: CLEAN DATABASE BEFORE SEEDING
DELETE FROM dispensers;
DELETE FROM staffs;
DELETE FROM stocks;
DELETE FROM visitors;
DELETE FROM grns;
DELETE FROM sales;
DELETE FROM sales_item;
-- END: CLEAN DATABASE

-- BEGIN: SEED DATABASE
COPY dispensers(dispenser_id, name)
FROM 'C:\Users\b\projs\mhcc\db_seeds\db-dispensers.csv' (FORMAT csv)

COPY staffs(staff_id, staff_name, uname, passwd, role, date_enrolled)
FROM 'C:\Users\b\projs\mhcc\db_seeds\db-staffs.csv' (FORMAT csv)

COPY stocks(stock_id, name, dispenser_id, uprice, quantity, date_expiry, staff_stocked) 
FROM 'C:\Users\b\projs\mhcc\db_seeds\db-stocks.csv' (FORMAT csv)

COPY visitors(v_id, name, address, tpno, dob, nic)
FROM 'C:\Users\b\projs\mhcc\db_seeds\db-visitors.csv' (FORMAT csv)

COPY grns(grn_id, date_returned, staff_returned)
FROM 'C:\Users\b\projs\mhcc\db_seeds\db-grns.csv' (FORMAT csv)

COPY sales(sales_id, v_id, date_sold, staff_id)
FROM 'C:\Users\b\projs\mhcc\db_seeds\db-sales.csv' (FORMAT csv)

COPY sales_item(sales_id, stock_id, quantity, uprice)
FROM 'C:\Users\b\projs\mhcc\db_seeds\db-sales_items.csv' (FORMAT csv)
-- END: SEED DATABASE

-- BEGIN: MISC
-- bootstrap FTS for stocks
ALTER TABLE stocks 
ADD search_tokens tsvector 
GENERATED ALWAYS AS (
    setweight(to_tsvector('simple',name), 'A') :: tsvector
) STORED;

ALTER TABLE dispensers
ADD search_tokens tsvector 
GENERATED ALWAYS AS (
    setweight(to_tsvector('simple',name), 'B') :: tsvector
) STORED;
-- END: MISC

-- BEGIN: CREATE INDEXES
CREATE INDEX idx_stock_date_expiry ON stocks(date_expiry);
CREATE INDEX idx_stock_name ON stocks USING GIN(search_tokens);
create index idx_dispensers_name on dispensers using GIN(search_tokens);
-- END: CREATE INDEXES