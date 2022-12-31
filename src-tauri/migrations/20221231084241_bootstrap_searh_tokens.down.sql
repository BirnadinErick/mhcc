ALTER TABLE IF EXISTS stocks 
    DROP COLUMN IF EXISTS search_tokens;

ALTER TABLE IF EXISTS dispensers 
    DROP COLUMN IF EXISTS search_tokens;