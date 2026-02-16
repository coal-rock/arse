#!/usr/bin/env bash

source ./.env
DB_PATH=$(echo $DATABASE_URL | sed s/sqlite://)
echo ".database" | sqlite3 $DB_PATH
sqlx migrate run
