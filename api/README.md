# Game API Service

## Requirements
- Rust tooolshain
- libssl
- protobuf compiler

## Installing Native Dependancies
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt-get install libssl-dev
sudo apt-get install protobuf-compiler
```
## To do Goals
### Current Phase
- [ ] Use Chrono time structs for all timestamps backed by unix epoch integers
- [ ] Fix Rust Enum mapping in SQLX and serde
- [ ] Build out core server data model and API (HP, MP, Class Levels, Current World, Zone, Position)
- [ ] Setup rust model to protobuf mapping.
- [ ] Create DB functions and API endpoints for users, characters, access_tokens, servers, and worlds.
- [ ] Create database query functions focused on characters, item instances, and collections.
- [ ] Create basic API endpoints focused on characters, item instances, and collections.
- [ ] Add more database CHECK constraints.
- [ ] Add a lot more data to initial data migration, with a focus on items, unlocks and companions. Potentially implement code based migrations.
- [ ] More consistant error structure.
- [ ] Better dependancy injection.
- [ ] Waiting period for User and Character deletion.
- [ ] Decide on good way to return Soulbound item to binder after a timer
### Future Phase
- [ ] JSON string passthrough without serialization.
- [ ] Replace sonyflake-rs with custom Snowflake ID style ID generator.

## Helpful Development Regexes
### Partial SQL CREATE TABLE to Rust Type Converter
```
[\S\s]*?CREATE TABLE\s+([a-z_])([a-z_]+)\s\( *\n([\S\s]*?)\n *\)\sSTRICT;
struct \U$1$2 {\n$3\n}\n\n
```
```
^\s+([a-z0-9_]+)\s+([A-Z]+)[,\s]+([^\s].*?)\s*,*$
    $1: $2,    // $3
```
### Find Nullable Columns in SQL
```
(?!.*?NOT NULL)(INTEGER|TEXT)[A-Za-z0-9\(\)_ ]*
```