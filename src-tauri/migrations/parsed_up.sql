CREATE TABLE staffs
(
    staff_id      SERIAL PRIMARY KEY,
    staff_name    CHARACTER VARYING (200) NOT NULL,
    uname         CHARACTER (20) NOT NULL UNIQUE,
    passwd        TEXT    NOT NULL,
    role          INTEGER NOT NULL DEFAULT 1,
    date_enrolled DATE NOT NULL DEFAULT NOW()
);

CREATE TABLE dispensers (
   	dispenser_id SERIAL PRIMARY KEY,
    dispenser_name CHARACTER VARYING (200) NOT NULL
);

CREATE TABLE stocks
(
    stock_id      SERIAL PRIMARY KEY,
    stock_name    CHARACTER VARYING (200) NOT NULL,
    dispenser_id  INTEGER NOT NULL,
    uprice        REAL    NOT NULL,
    quantity      INTEGER NOT NULL,
    date_expiry   DATE NOT NULL,
    staff_id INTEGER NOT NULL,
    FOREIGN KEY (staff_id)
        REFERENCES staffs (staff_id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION,
    FOREIGN KEY (dispenser_id)
        REFERENCES dispensers (dispenser_id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);

CREATE TABLE grns
(
    grn_id         SERIAL PRIMARY KEY,
    rec_timestamp  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    staff_id INTEGER NOT NULL,
    FOREIGN KEY (staff_id)
        REFERENCES staffs (staff_id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);

CREATE TABLE patients
(
    patient_id SERIAL PRIMARY KEY,
    patient_name    CHARACTER VARYING (200)    NOT NULL,
    address CHARACTER VARYING (200)    NOT NULL,
    tpno    CHARACTER VARYING (12)    NOT NULL,
    dob     DATE NOT NULL,
    nic     CHARACTER VARYING (20)    NOT NULL
);

CREATE TABLE sales
(
    sales_id  SERIAL PRIMARY KEY,
    patient_id INTEGER NOT NULL,
    sales_timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    staff_id  INTEGER NOT NULL,
    FOREIGN KEY (patient_id)
        REFERENCES patients (patient_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE,
    FOREIGN KEY (staff_id)
        REFERENCES staffs (staff_id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);

CREATE TABLE sales_item
(
    sales_id INTEGER NOT NULL,
    stock_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    uprice   REAL    NOT NULL,
    FOREIGN KEY (sales_id)
        REFERENCES sales (sales_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE,
    FOREIGN KEY (stock_id)
        REFERENCES stocks (stock_id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

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

CREATE INDEX idx_stock_name ON stocks USING GIN(search_tokens);
create index idx_dispensers_name on dispensers using GIN(search_tokens);

CREATE INDEX idx_stock_date_expiry ON stocks(date_expiry);