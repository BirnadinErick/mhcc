ALTER TABLE stocks
ADD search_tokens tsvector
GENERATED ALWAYS AS (
    setweight(to_tsvector('simple',stock_name), 'A') :: tsvector
) STORED;

ALTER TABLE dispensers
ADD search_tokens tsvector
GENERATED ALWAYS AS (
    setweight(to_tsvector('simple',dispenser_name), 'B') :: tsvector
) STORED;

ALTER TABLE patients
ADD search_tokens tsvector
GENERATED ALWAYS AS (
    setweight(to_tsvector('simple',patient_name), 'A') :: tsvector
) STORED;
