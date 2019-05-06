#!/bin/sh

cargo clean
cargo clippy -- -W clippy::all
