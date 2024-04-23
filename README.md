[![crates.io version badge](https://img.shields.io/crates/v/sportident.svg)](https://crates.io/crates/sportident)
[![Documentation](https://docs.rs/sportident/badge.svg)](https://docs.rs/sportident)

# Introduction

`sportident-rs` is a Rust crate that provides an implementation of the SportIdent reader protocol, allowing you to
communicate with SportIdent devices via a serial port connection. SportIdent is a widely used timing system in various
sports, such as orienteering, skiing, and running events.

# Features

- Poll card and read punch data (Supports: Si8, Si9, Si10, Si11, Siac, pCard, ComCard Up/Pro).

# Roadmap

- [ ] Configure SportIdent stations (set time, clear memory, etc.)
- [ ] Configure SportIdent cards (set name, email, etc.)

# Usage
Connect to a reader:
```rust
let reader = sportident::Reader::connect("/dev/ttyUSB0")
    .await
    .expect("failed to connect");
```
Poll card:
```rust
reader.poll_card()
      .await
      .expect("failed to poll card");
```
