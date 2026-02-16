#!/usr/bin/env bash
# Initializes database and generates TS bindings
./init_db.sh
cargo test export_bin
