#!/bin/bash -ex

if [ -z $TEST_DATABASE_URL ]; then
  echo "TEST_DATABASE_URL is not set. Aborting."
  echo "  Hint: You can set it to any local path; this script will create it there."
  echo "        e.g. `TEST_DATABASE_URL=test-database.sqlite make test`"
  exit 1;
fi

function teardown {
  rm $TEST_DATABASE_URL
}

function teardown_on_error {
  teardown
  exit 1
}

trap teardown_on_error ERR

cargo build

DATABASE_URL=$TEST_DATABASE_URL diesel setup

sqlite3 $TEST_DATABASE_URL ".read tests/fixtures.sql"

DATABASE_URL=$TEST_DATABASE_URL METAPHRASE_BIND=$TEST_METAPHRASE_BIND cargo test $TEST -- --nocapture

teardown

