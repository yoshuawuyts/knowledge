# postgres

## Attach to running DB
Install the client app:
```sh
$ apt-get install postgresql-client --yes
```

## Create database
```SQL
CREATE DATABASE "<db_name>";
```
```sh
$ createdb "<db_name>"
```
- https://www.postgresql.org/docs/9.0/static/sql-createdatabase.html
- https://www.postgresql.org/docs/9.1/static/app-createdb.html

## List all databases
```sh
# From within the REPL
postgres=# \l
```
```sh
# From shell
$ psql -U <username> -l
```


## Connect to specific database
```txt
postgres=# \c db_name
```

## Security
- http://lists.freebsd.org/pipermail/freebsd-performance/2005-February/001143.html
- http://momjian.us/main/blogs/pgblog/2012.html#June_6_2012
