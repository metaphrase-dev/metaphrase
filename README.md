# Metaphrase

_Metaphrase_ is a copy (writings, translations, etc.) manager that you can host on
your server or computer.

## Install

To make sure you don't run into trouble, make sure you have both `npm` and
`libsqlite3-dev` installed on your computer, as they are requirements (for
building the frontend part and the storage part of Metaphrase).

```console
make install
```

## Configure

Configuration is made with environment variables:

```console
DATABASE_URL=database.sqlite
DATABASE_BUSY_TIMEOUT=250
METAPHRASE_BIND=127.0.0.1:3000
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

To hack the frontend (after running `make install` at least):

```console
cd src/frontend
npm run dev
```

The frontend project is handled using [ViteJS](https://vitejs.dev/), so you
should have a very quick feedback loop on your changes. It's also configured
so it will mirror the calls to the backend engine as soon as it is run on
default URLs (backend on `localhost:3000` and frontend on `localhost:3100`).
Feel free to edit the `src/frontend/vite.config.js` locally to change those
values.

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
