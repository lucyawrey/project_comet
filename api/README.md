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
## To do Goals
### Current Phase
- [ ] Create database query functions focused on characters, item instances, and collections.
- [ ] Create basic API endpoints focused on characters, item instances, and collections.
- [ ] Add a lot more data to initial data migration, with a focus on items, unlocks and companions.
- [ ] Build out core player data model and API (HP, MP, Class Levels, Current World, Zone, Position)
### Future Phase
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