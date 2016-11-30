#!/bin/bash -ex

if [ -z $DATABASE_URL ]; then
  echo "DATABASE_URL is not set. Aborting."
  exit 1;
fi

command -v rustup >/dev/null 2>&1 || { echo 'I require `rustup` but it’s not installed. Install it with `curl https://sh.rustup.rs -sSf | sh`. Aborting.' >&2; exit 1; }

sudo apt install libsqlite3-dev

rustup override set nightly

cargo install diesel_cli --force

sudo npm install -g brunch
