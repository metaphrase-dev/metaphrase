install:
	tools/install.sh

migratedb:
	diesel migration run

build:
	cargo build
	(cd ./src/frontend && brunch build)

run: build
	cargo run

test:
	tools/test.sh
