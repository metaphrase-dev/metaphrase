install:
	tools/install.sh

migratedb:
	diesel migration run

build:
	tools/build.sh

run: build
	cargo run

test:
	tools/test.sh
