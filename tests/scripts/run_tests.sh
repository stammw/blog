#!/bin/bash

set -e

SCRIPT_DIR=$(dirname $0)

export DATABASE_URL=postgres://postgres:password@localhost/blog_test
export PGPASSWORD=password

diesel setup --database-url $DATABASE_URL
diesel migration --database-url $DATABASE_URL run
docker exec -i blog_pg psql -Upostgres blog_test < $SCRIPT_DIR/test_db.sql

RUST_BACKTRACE=1 cargo test $@
