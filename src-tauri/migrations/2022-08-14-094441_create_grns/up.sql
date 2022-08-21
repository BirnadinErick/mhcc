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