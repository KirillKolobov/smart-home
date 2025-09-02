@echo off
setlocal

REM Загрузить переменные из .env (только для простых переменных)
for /f "usebackq tokens=1,2 delims==" %%A in (".env") do set %%A=%%B

echo 🔌 Terminating active connections to %DB_NAME%...
psql -h %DB_HOST% -U %DB_USER% -p %DB_PORT% -d postgres -c "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '%DB_NAME%' AND pid <> pg_backend_pid();"

echo 👉 Dropping database %DB_NAME%...
psql -h %DB_HOST% -U %DB_USER% -p %DB_PORT% -d postgres -c "DROP DATABASE IF EXISTS %DB_NAME%;"

echo ✅ Dropped.

echo 🚀 Creating database %DB_NAME%...
psql -h %DB_HOST% -U %DB_USER% -p %DB_PORT% -d postgres -c "CREATE DATABASE %DB_NAME%;"

echo ✅ Created.

echo 📦 Running migrations...
set DATABASE_URL=%DATABASE_URL%
sqlx migrate run

echo ✅ All done!
endlocal