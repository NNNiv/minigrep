# minigrep

A simple version of `grep` written in Rust.

## Usage
```sh 
minigrep <pattern> <file name>
```
Running using `Cargo`:
```sh 
cargo run -- <pattern> <file name>
```
To perform case-insensitve searching use `IGNORE_CASE=1`
```sh 
IGNORE_CASE=1 minigrep <pattern> <file name>
```

`Cargo`:
```sh 
IGNORE_CASE=1 cargo run -- <pattern> <file name>
```
