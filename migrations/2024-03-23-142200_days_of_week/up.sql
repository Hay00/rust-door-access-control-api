-- Days of the week table
CREATE TABLE days_of_week (
    id INT AUTO_INCREMENT NOT NULL PRIMARY KEY,
    name VARCHAR(10) NOT NULL
);

-- Inserting days of the week
INSERT INTO days_of_week (name) VALUES ('Sunday');
INSERT INTO days_of_week (name) VALUES ('Monday');
INSERT INTO days_of_week (name) VALUES ('Tuesday');
INSERT INTO days_of_week (name) VALUES ('Wednesday');
INSERT INTO days_of_week (name) VALUES ('Thursday');
INSERT INTO days_of_week (name) VALUES ('Friday');
INSERT INTO days_of_week (name) VALUES ('Saturday');
