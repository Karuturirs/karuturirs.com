#!/usr/bin/env bash
set -e

echo "=== KaruturiRS WASM Build Script ==="

# Check if wasm-pack is installed and cargo is available
if command -v wasm-pack >/dev/null 2>&1; then
  echo "Found wasm-pack! Building Rust WebAssembly package..."
  cd rust_wasm
  wasm-pack build --target web --release --out-dir ../public/pkg
  cd ..
  echo "WASM compilation complete!"
else
  echo "wasm-pack not found in current environment."
  if [ -f "public/pkg/karuturirs_wasm_bg.wasm" ] && [ -f "public/pkg/karuturirs_wasm.js" ]; then
    echo "Found pre-built WebAssembly artifacts in public/pkg/. Ready for deployment!"
  else
    echo "Attempting to install wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    cd rust_wasm
    wasm-pack build --target web --release --out-dir ../public/pkg
    cd ..
  fi
fi

echo "Build finished successfully. Publishing directory: public/"
