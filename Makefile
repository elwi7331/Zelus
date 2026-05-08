.PHONY: clean

all:
	cargo build --release --manifest-path=chronos/Cargo.toml
	cp chronos/target/release/chronos scripts/chronos

clean:
	cargo clean --manifest-path=chronos/Cargo.toml
