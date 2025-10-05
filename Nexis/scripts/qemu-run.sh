#!/bin/bash
cargo build --target x86_64-nexis.json
qemu-system-x86_64 -kernel target/x86_64-nexis/debug/nexis
