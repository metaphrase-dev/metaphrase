#!/bin/bash -ex

function teardown {
  kill -SIGTERM $LUGH_PID
}

function teardown_on_error {
  teardown
  exit 1
}

trap teardown_on_error ERR

cargo build

diesel setup

LUGH_BIND=127.0.0.1:3100 target/debug/lugh &

LUGH_PID=$!

LUGH_BIND=127.0.0.1:3100 cargo test

teardown

