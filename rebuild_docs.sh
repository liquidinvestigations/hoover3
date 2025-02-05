#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

mkdir -p target/doc

rm -rf "target/doc/*"

cargo doc \
    --no-deps \
    --document-private-items \
    --workspace \
    --bins \
    --lib \
    --examples \
    --locked \
    --keep-going

du -hd1 target/doc
