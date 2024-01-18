CREATE TABLE IF NOT EXISTS Seats (
    seat_id INTEGER PRIMARY KEY,
    available BOOLEAN NOT NULL,
    other_info TEXT
);

CREATE TABLE IF NOT EXISTS Users (
    user_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    user_role TEXT NOT NULL,
    verified BOOLEAN NOT NULL,
    verification_token TEXT,
    points INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS UserInfos (
    user_id INTEGER PRIMARY KEY,
    user_name TEXT,
    phone_number INTEGER,
    id TEXT,
    FOREIGN KEY(user_id) REFERENCES Users(user_id)
);

CREATE TABLE IF NOT EXISTS RegistrationRequests (
    email TEXT PRIMARY KEY,
    user_name TEXT,
    phone_number INTEGER,
    id TEXT,
    registration_date TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Reservations (
    reservation_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    seat_id INTEGER NOT NULL,
    check_in_time TEXT,
    check_out_time TEXT,
    FOREIGN KEY(user_id) REFERENCES Users(user_id),
    FOREIGN KEY(seat_id) REFERENCES Seats(seat_id)
);

CREATE TABLE IF NOT EXISTS ReservationsHistory (
    reservation_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    seat_id INTEGER NOT NULL,
    check_in_time TEXT,
    check_out_time TEXT,
    FOREIGN KEY(user_id) REFERENCES Users(user_id),
    FOREIGN KEY(seat_id) REFERENCES Seats(seat_id)
);


CREATE TABLE IF NOT EXISTS UnavailableTimeSlots (
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    PRIMARY KEY (start_time, end_time)
);

CREATE TABLE IF NOT EXISTS BlackList (
    user_id INTEGER PRIMARY KEY,
    banned_at TEXT NOT NULL,
    unbanned_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES Users(user_id)
);