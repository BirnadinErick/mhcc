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