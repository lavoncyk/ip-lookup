# IP Lookup CLI Tool

### The Protocol
Application implements specific line-based request-response
protocol. It consists only of three commands:

* `LOAD`
* `LOOKUP <IPv4 address>`
* `EXIT`
​
#### LOAD
`LOAD` command loads the database into memory. The application responds
with the string `OK` once the database is fully loaded or `ERR` in case
of an error.
​
Example command execution:
```shell
> LOAD
< OK

> LOAD
< ERR
```
​
#### LOOKUP
`LOOKUP` performs geolocation lookup. The application responds with the
location formated as `<COUNTRY CODE>,<CITY>` or `ERR` in case of an error
or if no location was found.
​
Example command execution:
```shell
> LOOKUP 71.8.28.3
< US,Hammond

> LOOKUP 1.2.3.4.5
< ERR
```
​
#### EXIT
`EXIT` tells application to exit. The application responds with `OK`.
​
Example command execution:
```shell
> EXIT
< OK
```
​
