#!/bin/bash
cargo build --release
sudo cp target/release/tab /usr/local/bin/tab