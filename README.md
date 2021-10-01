[![Rust](https://github.com/rewe-digital/schema2000/actions/workflows/rust.yml/badge.svg)](https://github.com/rewe-digital/schema2000/actions/workflows/rust.yml) Schema 2000
===========

How to use

```shell
$ cat line_separated.json | cargo run
$ mosquitto_sub -t homeassistant/event | cargo run
$ kafkacat -b $KAFKA_BROKER_ADDRESS_LIST -t your_topic | cargo run
```
