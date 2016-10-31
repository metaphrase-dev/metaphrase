# Lugh

*Lugh* is a copy (writings, translations, etc.) manager that you can host on
your server or computer.

## Install
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

## Run
```
make run
```

## Development

### Frontend

The frontend embedded project is handled using [Brunch.io](http://brunch.io)
which handle all the hassle of modern JS development in a simple, clean way.

To hack the frontend, install npm, then:

```
# sudo npm install -g brunch
# cd src/frontend
# brunch watch
```

Brunch will take care of downloading all what is necessary to run the frontend
and build everything, then watch for your changes to build it to the
`src/frontend/public` folder where the Rust part will serve it.

To allow quick-bootstrap for the project, the `src/frontend/public` is committed
on purpose to make sure people can easily boot Lugh rapidly.
