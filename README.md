[![Rust](https://github.com/rewe-digital/schema2000/actions/workflows/rust.yml/badge.svg)](https://github.com/rewe-digital/schema2000/actions/workflows/rust.yml) Schema 2000
===========

Schema2000 is a tool that parses exsiting [JSON](https://www.json.org/json-en.html) documents and tries to derive a [JSON schema](https://json-schema.org/) from these documents.

Currently, Schema2000 is configuration-less command line tool that excepts line-separated JSON documents from `stdin` and emits the derived schema to `stdout` once the input is consumed.

How to install
--------------

Download the [latest binaries for your operating system](https://github.com/rewe-digital/schema2000/releases), and add them to your `$PATH` and make them executable.

### macOS specific installation

macOS will not open the app as the developer can not be verified. As a work-around, open the folder containing `schema2000_macOS` in Finder, "ctrl-click" on `schema2000_macOS` and click on the menu item "open".  You can then confirm to open the app.  This will open a terminal app session, which you can simply close.

Usage
-----

Consume a file with line separated JSON documents:

```shell
$ cat line_separated.json | schema2000
```

Consume via MQTT (using [Eclipse Mosquitto](https://mosquitto.org/)):

```shell
$ mosquitto_sub -t homeassistant/event | schema2000
```

Consume from Kafka (using [kcat](https://github.com/edenhill/kcat#readme)):

```shell
$ kafkacat -b $KAFKA_BROKER_ADDRESS_LIST -t your_topic | schema2000
```

### Verify schemas

You may use any JSON schema validator to validate the input documents with the derived schema. This example uses [yajsv](https://github.com/neilpa/yajsv):

```shell
yajsv -s schema.json line_separated.json
```
