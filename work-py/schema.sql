CREATE TABLE batch (
    uuid TEXT PRIMARY KEY,
    imsi TEXT NOT NULL,
    msisdn TEXT NOT NULL,
    vlr_number TEXT NOT NULL
);


CREATE TABLE imsi (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imsi TEXT NOT NULL UNIQUE
);


CREATE TABLE msisdn (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    msisdn TEXT NOT NULL UNIQUE
);

CREATE TABLE calendar_dim (
    date_key DATE PRIMARY KEY,
    year INTEGER NOT NULL,
    quarter INTEGER NOT NULL,
    month INTEGER NOT NULL,
    month_name TEXT NOT NULL,
    day INTEGER NOT NULL,
    day_of_week INTEGER NOT NULL,
    day_name TEXT NOT NULL,
    is_weekend BOOLEAN NOT NULL,
    is_holiday BOOLEAN NOT NULL
);