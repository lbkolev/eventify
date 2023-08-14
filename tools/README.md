## Function signatures
It takes around ~130 minutes to fetch all the publicly available ethereum function signtures (thanks https://4byte.directory) and insert them into a csv, that's later copied into table `function_signature`.

### Benchmark for ~10400 pages (100 signatures per page)
```
lua function_signatures.lua  97.75s user 18.05s system 1% cpu 2:12:11.80 total
```
