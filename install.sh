#!/bin/bash
cargo build --release
sudo mkidr -p /usr/local/bin/
sudo cp target/release/tab /usr/local/bin/tab