install:
	curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
	cargo install diesel_cli
	sudo apt install libsqlite3-dev

migratedb:
	diesel migration run

build:
	cargo build

run:
	cargo run
