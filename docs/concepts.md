# Core Concepts

## Simple

`zedis` only has 7 commands:  

`GET`, `SET`, `DEL`, `KEYS`, `PRE`, `FLUSH` and `CLEAR`

## Lightweight

`zedis` is embedded and brokerless and tiny (`< 1.5 MBs`)

Minimal CPU and Memory resources are used, only uses `<50 MBs` when reading/writing to a `>1 GBs` datastore.

## Accessible

`zedis` can be connected to from any language. We offically support [Python](https://github.com/drbh/pyzedis) however, since `zedis` is build with zmq, we can connect with any zmq client. The follow languages have fully supported zmq clients.

```C++ | C# | Clojure | CL | Delphi | Erlang | F# | Felix | Go | Haskell | Haxe | Java | Lua | Node.js | Objective-C | Perl | PHP | Python | Q | Racket | Ruby | Scala | Tcl | Ada | Basic```

`zedis` can supports multi-client connections, which means you can read and write at the same time 🙌. Unlike other embedded databases you can open a GUI while you run your program since the DB does not lock for a single client.

## Fully Featured 

`zedis` comes with a toolkit to make development easy and painless.

- zedis-cli (CLI REPL)
- zedis-interface (REST API and GUI)
- zedis (ENCRYPTION -- in development)