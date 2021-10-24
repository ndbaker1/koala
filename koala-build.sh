#!/bin/sh
cd koala; cargo build && cd ../koala-wasm; wasm-pack build --target web