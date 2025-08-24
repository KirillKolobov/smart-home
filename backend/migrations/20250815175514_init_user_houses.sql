CREATE TABLE user_houses (
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    house_id BIGINT REFERENCES houses(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, house_id)
);