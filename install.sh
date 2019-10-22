#!/bin/bash
cargo build --release
sudo mkdir -p /usr/local/bin/
sudo cp target/release/tab /usr/local/bin/tab