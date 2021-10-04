[![Rust](https://github.com/rewe-digital/schema2000/actions/workflows/rust.yml/badge.svg)](https://github.com/rewe-digital/schema2000/actions/workflows/rust.yml) Schema 2000
===========

How to install
--------------

Download the [latest binaries for your operating system](https://github.com/rewe-digital/schema2000/releases), and add them to your `$PATH` and make them executable.

Usage
-----

Schema2000 currently is configuration-less, and excepts line-separated-json from stdin and emits the schema to stdout 
once stdin is consumed. 

### How to use


```shell
$ cat line_separated.json | schema2000
$ mosquitto_sub -t homeassistant/event | schema2000
$ kafkacat -b $KAFKA_BROKER_ADDRESS_LIST -t your_topic | schema2000
```

### Verify schemas

You may use any JSON schema validator, this example uses [yajsv](https://github.com/neilpa/yajsv).

```shell
yajsv -s ${TOPIC}_schema.json line_separated.json
```
