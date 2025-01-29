# Game API Service

## Requirements
- Rust
- libssl
- protobuf

## Installing Native Dependancies
```sh
sudo apt-get install libssl-dev
sudo apt-get install protobuf-compiler
```
## Future Goals
- [ ] Replace sonyflake-rs with custom Snowflake ID style ID generator.

## Model Code Gen Regexes
```
[\S\s]*?CREATE TABLE\s+([a-z_])([a-z_]+)\s\( *\n([\S\s]*?)\n *\)\sSTRICT;
struct \U$1$2 {\n$3\n}\n\n
```
```
^\s+([a-z0-9_]+)\s+([A-Z]+)[,\s]+([^\s].*?)\s*,*$
    $1: $2,    // $3
```