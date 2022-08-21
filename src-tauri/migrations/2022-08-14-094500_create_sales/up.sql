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
