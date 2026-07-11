#!/usr/bin/env bash
#
# Deletes the database referenced by backend/.env and builds a fresh one
# from backend/database-init.sql.
#
# Usage: ./reset-db.sh
set -euo pipefail

cd "$(dirname "$0")"

DB_URL="$(grep -E '^DATABASE_URL=' backend/.env | head -n1 | cut -d= -f2-)"
if [ -z "${DB_URL}" ]; then
  echo "DATABASE_URL not set in backend/.env" >&2
  exit 1
fi

# Split the URL into the database name and a maintenance connection (the "postgres"
# database), because you cannot DROP a database you are connected to.
DB_NAME="${DB_URL##*/}"; DB_NAME="${DB_NAME%%\?*}"
BASE="${DB_URL%/*}"
ADMIN_URL="${BASE}/postgres"

echo "Deleting and rebuilding database '${DB_NAME}'..."

# Kick off any lingering connections (e.g. the backend) so the drop can proceed.
psql "${ADMIN_URL}" -v ON_ERROR_STOP=1 -c \
  "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '${DB_NAME}' AND pid <> pg_backend_pid();" >/dev/null

psql "${ADMIN_URL}" -v ON_ERROR_STOP=1 -c "DROP DATABASE IF EXISTS \"${DB_NAME}\";"
psql "${ADMIN_URL}" -v ON_ERROR_STOP=1 -c "CREATE DATABASE \"${DB_NAME}\";"

# Build the new schema (including the new herb_quest tables).
psql "${DB_URL}" -v ON_ERROR_STOP=1 -f backend/database-init.sql

echo "Done. '${DB_NAME}' deleted and rebuilt from backend/database-init.sql"
