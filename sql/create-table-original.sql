CREATE TABLE staffs
(
    staff_id      INTEGER PRIMARY KEY,
    name          TEXT    NOT NULL,
    uname         TEXT    NOT NULL UNIQUE,
    passwd        TEXT    NOT NULL,
    role          INTEGER NOT NULL DEFAULT 1,
    date_enrolled INTEGER NOT NULL
);

CREATE TABLE stocks
(
    stock_id      INTEGER PRIMARY KEY,
    name          TEXT    NOT NULL,
    dispenser     TEXT    NOT NULL,
    uprice        REAL    NOT NULL,
    quantity      INTEGER NOT NULL,
    date_expiry   INTEGER NOT NULL,
    staff_stocked INTEGER NOT NULL,
    FOREIGN KEY (staff_stocked)
        REFERENCES staffs (staff_id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);

CREATE TABLE grns
(
    grn_id         INTEGER PRIMARY KEY,
    date_returned  INTEGER NOT NULL,
    staff_returned INTEGER NOT NULL,
    FOREIGN KEY (staff_returned)
        REFERENCES staffs (staff_id)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);

CREATE TABLE visitors
(
    v_id    INTEGER PRIMARY KEY,
    name    TEXT    NOT NULL,
    address TEXT    NOT NULL,
    tpno    TEXT    NOT NULL,
    dob     INTEGER NOT NULL,
    nic     TEXT    NOT NULL
);

CREATE TABLE sales
(
    sales_id  INTEGER PRIMARY KEY,
    v_id      INTEGER NOT NULL,
    date_sold INTEGER NOT NULL,
    staff_id  INTEGER NOT NULL,
    FOREIGN KEY (v_id)
        REFERENCES visitor (v_id)
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