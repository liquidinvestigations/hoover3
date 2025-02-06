#!/bin/bash
cd "$(dirname "${BASH_SOURCE[0]}")"
set -ex

if [ -z "$(git status --porcelain)" ]; then
    cargo fmt

    git add .
    git status
    git commit -m "fmt.sh: cargo fmt $(date)" || true

    cargo fix   --allow-dirty || true

    git add .
    git status
    git commit -m "fmt.sh: cargo fix $(date)" || true
    git push

else
    git status
    set +x
    echo
    echo "WORKING DIRECTORY NOT CLEAN"
    echo "PLZ COMMIT CHANGES"
    exit 66
fi