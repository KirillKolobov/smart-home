#!/bin/bash
set -e

set -o allexport
source .env
set +o allexport


echo "🔌 Terminating active connections to $DB_NAME..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -U $DB_USER -p $DB_PORT -d postgres -c \
"SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$DB_NAME' AND pid <> pg_backend_pid();"

echo "👉 Dropping database $DB_NAME..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -U $DB_USER -p $DB_PORT -d postgres -c "DROP DATABASE IF EXISTS $DB_NAME;"

echo "✅ Dropped."

echo "🚀 Creating database $DB_NAME..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -U $DB_USER -p $DB_PORT -d postgres -c "CREATE DATABASE $DB_NAME;"

echo "✅ Created."

echo "📦 Running migrations..."
DATABASE_URL=$DATABASE_URL sqlx migrate run

echo "✅ All done!"