#!/bin/bash -ex

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

DATABASE_URL=$TEST_DATABASE_URL LUGH_BIND=$TEST_LUGH_BIND target/debug/lugh &

LUGH_PID=$!

DATABASE_URL=$TEST_DATABASE_URL LUGH_BIND=$TEST_LUGH_BIND cargo test

teardown

