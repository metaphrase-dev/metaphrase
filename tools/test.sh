#!/bin/bash -ex

if [ -z $TEST_DATABASE_URL ]; then
  echo "TEST_DATABASE_URL is not set. Aborting."
  exit 1;
fi

if [ -z $TEST_LUGH_BIND ]; then
  echo "TEST_LUGH_BIND is not set. Aborting."
  exit 1;
fi

if [ -z "$TEST_LUGH_AVAILABLE_LOCALES" ]; then
  echo "TEST_LUGH_AVAILABLE_LOCALES is not set. Aborting."
  exit 1;
fi

function teardown {
  kill -SIGTERM $LUGH_PID
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

DATABASE_URL=$TEST_DATABASE_URL LUGH_BIND=$TEST_LUGH_BIND LUGH_AVAILABLE_LOCALES=$TEST_LUGH_AVAILABLE_LOCALES target/debug/lugh &

LUGH_PID=$!

DATABASE_URL=$TEST_DATABASE_URL LUGH_BIND=$TEST_LUGH_BIND LUGH_AVAILABLE_LOCALES=$TEST_LUGH_AVAILABLE_LOCALES cargo test $TEST -- --nocapture

teardown

