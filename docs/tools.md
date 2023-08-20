# Tools

## [Init DB](../tools/init_db.sh)
Spawns a local pgsql instance running in docker & runs all the database migrations there are, essentially bootstrapping a local dev environment.

## [Function signatures](./../tools/function_signatures.lua)
It takes around ~130 minutes on a `Macbook air 2020, 16GB RAM, M1 chip` to *synchronously* fetch all the publicly available ethereum function signtures (thanks https://4byte.directory) and insert them into a csv that we use in the initial database setup to insert into table `public.function_signature`.

```
lua function_signatures.lua  97.75s user 18.05s system 1% cpu 2:12:11.80 total
```
