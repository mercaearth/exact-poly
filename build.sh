#!/bin/bash
set -e

wasm-pack build --target bundler --release
echo "exact-poly WASM built successfully"
