#!/bin/bash
cp static/* target/wasm32-unknown-unknown/release/taskigt.{js,wasm} .
echo git add .
echo git commit
