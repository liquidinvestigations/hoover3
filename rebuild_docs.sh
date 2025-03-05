#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

mkdir -p target/doc

touch target/doc/.some_data || ( echo "Failed to touch `target/doc/.some_data` -- check permisisons" && exit 1 )
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
