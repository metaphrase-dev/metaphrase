install:
	curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
	cargo install diesel_cli
	sudo apt install libsqlite3-dev
	sudo npm install -g brunch

migratedb:
	diesel migration run

build:
	cargo build
	(cd ./src/frontend && brunch build)

run: build
	cargo run
