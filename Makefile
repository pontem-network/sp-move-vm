build_wasm_no_std:
	cd mvm; cargo +nightly build --package mvm --target wasm32-unknown-unknown --no-default-features --features="sp_check"

build_wasm_std:
	cd mvm; cargo +nightly build --package mvm --no-default-features --features="std,sp_check"

clippy:
	cargo clippy

tests:
	cargo test --all --tests --no-fail-fast -- --test-threads=4 --nocapture

pre-compile: clippy tests build_wasm_std build_wasm_no_std
	cargo build

gas_bench:
	cargo test --release --test gas_bench gas_bench  --features "bench" -- --nocapture