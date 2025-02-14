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
- [ ] Rust level defaults for datbase tables and fields.
- [ ] Create core user, access_token, and authentication logic.
- [ ] Implement all toml table importers and support for enum string names in toml.
- [ ] Build out core server data model and API schema (HP, MP, Class Levels, Current World, Zone, Position)
- [ ] Setup rust model to protobuf mapping?
- [ ] Create database query functions focused on characters, item instances, and collections.
- [ ] Create basic API endpoints focused on characters, item instances, and collections.
- [ ] Make toml data importers diff based.
- [ ] Add more database CHECK constraints and triggers
- [ ] More consistant error structure.
- [ ] Better dependancy injection.
- [ ] Waiting period for User and Character deletion.
- [ ] Decide on good way to return Soulbound item to binder after a timer
### Future Phase
- [ ] JSON string passthrough without serialization.

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