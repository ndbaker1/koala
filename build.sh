#!/bin/sh

cd koala
cargo build --release --all-features
cd ..
cd koala-wasm
wasm-pack build --target web
cd ..
npm install -f
[ -z $BASE_HREF ] && npm run build || npm run build -- --base=$BASE_HREF