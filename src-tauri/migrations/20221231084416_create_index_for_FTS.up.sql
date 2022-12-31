CREATE INDEX idx_stock_name ON stocks USING GIN(search_tokens);
create index idx_dispensers_name on dispensers using GIN(search_tokens);