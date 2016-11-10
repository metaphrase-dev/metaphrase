#!/bin/bash -ex

cargo build

diesel setup

LUGH_BIND=127.0.0.1:3100 target/debug/lugh &

LUGH_PID=$!

LUGH_BIND=127.0.0.1:3100 cargo test

kill -SIGTERM $LUGH_PID

