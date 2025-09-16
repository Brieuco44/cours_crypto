.PHONY: cesar cesar-test

# TP Cesar en Rust
cesar: 
	cd cesar && cargo build --release

cesar-test: 
	cd cesar && cargo test