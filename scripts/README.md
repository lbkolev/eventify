# Tools

## [Function signatures](./fn-sig.lua)

It takes around ~130 minutes on a `Macbook air 2020, 16GB RAM, M1 chip` to _synchronously_ fetch all the publicly available ethereum function signtures (thanks https://4byte.directory) into a csv, that's inserted into `eth.function_signature` during the initial database setup.

```
lua function_signatures.lua  97.75s user 18.05s system 1% cpu 2:12:11.80 total
```
