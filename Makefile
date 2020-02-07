main: src
	cargo build --release --bin cmkv
	cp target/release/cmkv .
