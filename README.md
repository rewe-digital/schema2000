[![Rust](https://github.com/rewe-digital-misc/hackdays2021-schema-2000/actions/workflows/rust.yml/badge.svg)](https://github.com/rewe-digital-misc/hackdays2021-schema-2000/actions/workflows/rust.yml) Schema 2000
===========

How to install
--------------

Download the [latest binaries for your operating system](https://github.com/rewe-digital-misc/hackdays2021-schema-2000/releases), and add them to your `$PATH` and make them executable.

### macOS specific installation

macOS will not open the app as the developer can not be verified. As a work-around, open the folder containing `schema2000_macOS` in Finder, "ctrl-click" on `schema2000_macOS` and click on the menu item "open".  You can then confirm to open the app.  This will open a terminal app session, which you can simply close.

Usage
-----

Schema2000 currently is configuration-less, and excepts line-separated-json from stdin and emits the schema to stdout once stdin is consumed.

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
