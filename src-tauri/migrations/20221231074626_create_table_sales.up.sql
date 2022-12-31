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