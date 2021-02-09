# Lugh

_Lugh_ is a copy (writings, translations, etc.) manager that you can host on
your server or computer.

[![Build Status](https://travis-ci.org/rlustin/lugh.svg?branch=master)](https://travis-ci.org/rlustin/lugh)

## Install

To make sure you don't run into trouble, make sure you have both `npm` and
`libsqlite3-dev` installed on your computer, as they are requirements (for
building the frontend part and the storage part of Lugh).

```console
make install
```

## Configure

Configuration is made with environment variables:

```console
DATABASE_URL=database.sqlite
DATABASE_BUSY_TIMEOUT=250
LUGH_BIND=127.0.0.1:3000
```

## Migrate database

```console
make migratedb
```

## Build (backend & frontend)

```console
make build
```

## Run

```console
make run
```

## Development

### Frontend

The frontend embedded project is handled using [Brunch.io](http://brunch.io)
which handle all the hassle of modern JS development in a simple, clean way.

To hack the frontend:

```console
cd src/frontend
brunch watch
```

Brunch will take care of downloading all what is necessary to run the frontend
and build everything, then watch for your changes to build it to the
`src/frontend/public` folder where the Rust part will serve it.

## Tests

So that you do not conflict with the development, use these environment variables:

```console
export TEST_DATABASE_URL=test-database.sqlite
```

You can run the tests with `make test`.

To run a specific test, use `make test TEST=test_name`. Example:

```console
make test TEST=api::v1::translations::tests::test_validate_with_success
```
