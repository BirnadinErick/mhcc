CREATE TABLE staffs
(
    staff_id      SERIAL PRIMARY KEY,
    staff_name    CHARACTER VARYING (200) NOT NULL,
    uname         CHARACTER (10) NOT NULL UNIQUE,
    passwd        TEXT    NOT NULL,
    role          INTEGER NOT NULL DEFAULT 1,
    date_enrolled DATE NOT NULL DEFAULT NOW()
);