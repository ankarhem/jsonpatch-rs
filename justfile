build:
    wasm-pack build --target web --out-dir ./build/browser && pnpm rimraf ./build/browser/package.json
    wasm-pack build --target nodejs --out-dir ./build/node && pnpm rimraf ./build/node/package.json
