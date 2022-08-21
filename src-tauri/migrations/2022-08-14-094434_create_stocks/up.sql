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