Project start

sudo -u postgres psql
CREATE DATABASE `db_name`;
CREATE USER `user_name` WITH PASSWORD `password`;
GRANT ALL PRIVILEGES ON DATABASE `db_name` TO `user_name`;
ALTER USER `user_name` WITH SUPERUSER;

sqlx migrate run
