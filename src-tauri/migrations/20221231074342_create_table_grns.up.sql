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