CREATE TABLE IF NOT EXISTS rounds (
    id INTEGER PRIMARY KEY,
    number INTEGER NOT NULL,
    started_at REAL NOT NULL,
    ended_at REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS teams (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS services (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    check_name TEXT NOT NULL,
    check_configuration TEXT NOT NULL,
    weight INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS service_checks (
    round_id   INTEGER NOT NULL,
    team_id    INTEGER NOT NULL,
    service_id INTEGER NOT NULL,

    status     INTEGER NOT NULL CHECK (status IN (0, 1, 2)), -- 0: Down, 1: Degraded, 2: Up
    message    TEXT,
    timestamp  INTEGER NOT NULL,

    PRIMARY KEY (round_id, team_id, service_id),

    FOREIGN KEY (round_id) REFERENCES rounds(id) ON DELETE CASCADE,
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (service_id) REFERENCES services(id) ON DELETE CASCADE
);
