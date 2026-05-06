#!/bin/bash
set -e

rm -rf pkg

wasm-pack build --target bundler --out-dir pkg/bundler --release
wasm-pack build --target nodejs  --out-dir pkg/node    --release
wasm-pack build --target web     --out-dir pkg/web     --release

for dir in pkg/bundler pkg/node pkg/web; do
  : > "$dir/.npmignore"
  rm -f "$dir/.gitignore" "$dir/package.json" "$dir/README.md" "$dir/LICENSE"
done

echo "exact-poly WASM built (bundler + node + web targets)"
