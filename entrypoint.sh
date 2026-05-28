#!/bin/bash
cd rust-api && cargo run --offline &
cd nuxt && npx --offline serve -l 5173
#  && cargo build --offline &&