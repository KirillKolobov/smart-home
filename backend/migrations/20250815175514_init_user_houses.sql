CREATE TABLE user_houses (
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    house_id INTEGER REFERENCES houses(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, house_id)
);