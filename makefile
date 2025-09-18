.PHONY: cesar cesar-test vigenere vigenere-test matrix matrix-test all-test

# TP Cesar en Rust
cesar: 
	cd cesar && cargo build --release

cesar-test: 
	cd cesar && cargo test

# TP Vigenere en Rust
vigenere: 
	cd vigenere && cargo build --release

vigenere-test: 
	cd vigenere && cargo test

# TP Matrix en Rust
matrix: 
	cd matrix && cargo build --release

matrix-test: 
	cd matrix && cargo test

all-test:  cesar-test vigenere-test matrix-test