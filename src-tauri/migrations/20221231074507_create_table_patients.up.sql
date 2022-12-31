CREATE TABLE patients
(
    patient_id SERIAL PRIMARY KEY,
    patient_name    CHARACTER VARYING (200)    NOT NULL,
    address CHARACTER VARYING (200)    NOT NULL,
    tpno    CHARACTER VARYING (12)    NOT NULL,
    dob     DATE NOT NULL,
    nic     CHARACTER VARYING (20)    NOT NULL
);