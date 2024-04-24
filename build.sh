#!/bin/bash

RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target=x86_64-unknown-linux-gnu
upx target/x86_64-unknown-linux-gnu/release/initd

cp target/x86_64-unknown-linux-gnu/release/initd ~/Projekte/initramfs/src/initd
cp -r openinit.d ~/Projekte/initramfs/src/etc/
