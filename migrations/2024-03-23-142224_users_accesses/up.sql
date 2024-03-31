-- Table to control the access of the users to the system, based on composite keys (user_id, day_of_week) and the time range (start, end)

CREATE TABLE users_accesses (
    user_id INT NOT NULL,
    day_of_week INT NOT NULL,
    start TIME NOT NULL,
    end TIME NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE,
    FOREIGN KEY (day_of_week)
        REFERENCES days_of_week (id)
        ON DELETE CASCADE,
    PRIMARY KEY (user_id , day_of_week)
);
