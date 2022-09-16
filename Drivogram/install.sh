#!/usr/bin/bash

cargo build --release
chmod +x target/release/drivogram
mv target/release/drivogram $HOME/.cargo/bin/
