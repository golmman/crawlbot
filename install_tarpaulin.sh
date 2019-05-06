#!/bin/bash

echo 'see "https://github.com/xd009642/tarpaulin"'

RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin
