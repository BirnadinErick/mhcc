CREATE INDEX idx_stock_name ON stocks USING GIN(search_tokens);
CREATE INDEX idx_dispensers_name ON dispensers USING GIN(search_tokens);
