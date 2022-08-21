CREATE TABLE staffs
(
    staff_id      INTEGER PRIMARY KEY,
    name          TEXT    NOT NULL,
    uname         TEXT    NOT NULL UNIQUE,
    passwd        TEXT    NOT NULL,
    role          INTEGER NOT NULL DEFAULT 1,
    date_enrolled INTEGER NOT NULL
);