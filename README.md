# Lugh

*Lugh* is a copy (writings, translations, etc.) manager that you can host on
your server or computer.

[![Build Status](https://travis-ci.org/rlustin/lugh.svg?branch=master)](https://travis-ci.org/rlustin/lugh)

## Install

To make sure you don't run into trouble, make sure you have both `npm` and
`libsqlite3-dev` installed on your computer, as they are requirements (for
building the frontend part and the storage part of Lugh).

```
make install
```

## Configure
Configuration is made in `.env` file:

```
DATABASE_URL=database.sqlite
```

## Migrate database
```
make migratedb
```

## Build (backend & frontend)

```
make build
```

## Run
```
make run
```

## Development

### Frontend

The frontend embedded project is handled using [Brunch.io](http://brunch.io)
which handle all the hassle of modern JS development in a simple, clean way.

To hack the frontend:

```
$ cd src/frontend
$ brunch watch
```

Brunch will take care of downloading all what is necessary to run the frontend
and build everything, then watch for your changes to build it to the
`src/frontend/public` folder where the Rust part will serve it.
