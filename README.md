# kv-store

`kv-store` is a bare-bones implementation of a durable key-value store. It supports GET, SET and DELETE operations.

## High-level technical information

`kv-store` is a key-value implementation using ideas take from Martin Kleppmann's fantastic
book [Designing Data-Intensive Applications](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781491903063/)
and [Bitcask](https://github.com/basho/bitcask).

It performs single-threaded access to a log file which is append-only. Entries are added using a binary format specified
in the next section. Thus, write operations are very fast as it's translated to an append operation on a file.

Read operations are sped up by maintaining an in-memory hash-table index. For each key, the index stores the location of
the entry in the log file, the key length as well as the value length. Thus, read operations can be performed with a
single seek. If the OS has cached the log file in the in-memory file cache, no disk access is needed to server read
requests.

Deletions are represented using tombstones.

## Log file format

The binary format of the log file is as trivial as it can be. Each entry to the key-value store is appended to a file.
The first 8 bytes store the key-length in big endianness. The next 8 bytes store the value-length in big endianness. The
next <key-length> bytes store the key in utf-8 format. And lastly, <value-length> bytes store the value in utf-8 format.
Tombstones are represented using the NULL (`0x0_u8`) byte.

```
u64         u64           [u8]        [u8]          
key-length  value-length  key(utf-8)  value(utf-8)  
```

## Supported Features

- TCP server accepting one connection at a time
- GET, SET and DELETE operations
- Durability
- String data type

## Missing Features

- Crash recovery
- Log compaction
- Multithreading
- ... a gazillion more things 😆

## Developing

```shell
# Start up the server
cargo run
```

```shell
# IN a separate shell, connect to the server
nc -v 127.0.0.1 7878
> SET key value
> success
> GET key
> value
> DELETE key
> success
> GET key
> no value stored
```



