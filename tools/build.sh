#!/bin/bash -ex

# Cargo
if [[ "$OSTYPE" == "darwin"* ]]; then
	# macOS
	# Let’s assume openssl was installed via homebrew
	export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
	export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
fi
cargo build

# Brunch
cd ./src/frontend && npm run build
