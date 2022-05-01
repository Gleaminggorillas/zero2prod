#!/usr/bin/env bash
set -x
set -eo pipefail

# check for user, else default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# check for password, else default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# check for custom db, else default to newsletter
DB_NAME="${POSTGRES_DB:=newsletter}"
# check if a custom port has been set, else default to 5432
DB_PORT="${POSTGRES_PORT:=5432}"

# Check psql is installed

# if ! [ -x "$(command -v psql)" ]; then
# 	echo >&2 "Error: psql is not installed."
# 	exit 1
# fi

# Check sqlx is installed

if ! [ -x "$(command -v sqlx)" ]; then
 	echo >&2 "Error: sqlx is not installed.";
 	echo >&2 "Use:";
 	echo >&2 "	cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres";
 	echo >&2 "to install it";
	exit 1;
fi

# Launch postgres using docker
# Allow to skip if a dockerized postgres is running
if [[ -z "${SKIP_DOCKER}" ]];
then
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000;
fi

# ping postgres until its accepting commands
# REQUIRES FIX
# export PGPASSWORD="${DB_PASSWORD}";
# until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do >&2 echo "Postgres is still unavailable - sleeping";
# 	sleep 1;
# done
# 
# >&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now.";

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME};
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!";

