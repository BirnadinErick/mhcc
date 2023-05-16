CREATE TABLE staffs
(
    staff_id      SERIAL PRIMARY KEY,
    staff_name    CHARACTER VARYING (200) NOT NULL,
    uname         CHARACTER (10) NOT NULL UNIQUE,
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
    name          CHARACTER VARYING (200)    NOT NULL,
    dispenser_id  INTEGER    NOT NULL,
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
    date_returned  TIMESTAMP NOT NULL DEFAULT NOW(),
    staff_returned INTEGER NOT NULL,
    FOREIGN KEY (staff_returned)
        REFERENCES staffs (staff_id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);

CREATE TABLE visitors
(
    v_id    SERIAL PRIMARY KEY,
    name    CHARACTER VARYING (200)    NOT NULL,
    address CHARACTER VARYING (200)    NOT NULL,
    tpno    CHARACTER VARYING (12)    NOT NULL,
    dob     DATE NOT NULL,
    nic     CHARACTER VARYING (20)    NOT NULL
);

CREATE TABLE sales
(
    sales_id  SERIAL PRIMARY KEY,
    v_id      INTEGER NOT NULL,
    date_sold TIMESTAMP NOT NULL DEFAULT NOW(),
    staff_id  INTEGER NOT NULL,
    FOREIGN KEY (v_id)
        REFERENCES visitorS (v_id)
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
