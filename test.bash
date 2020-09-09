#!/bin/bash

set -ex

# rust-docs

# bindir=$(dirname $(rustup which rustc))
# sharedir=$bindir/../share

# trait_join=$sharedir/doc/rust/html/alloc/slice/trait.Join.html

# hack, unreliable
index=$(cargo doc --open 2>&1| rg inde | awk '{print $2}')

trait_path=(dirname $index)/trait.Join.html

exec grep -qF impl-Join $trait_path
