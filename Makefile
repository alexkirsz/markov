cmkv: FORCE
	cargo build --release --bin cmkv
	cp target/release/cmkv .

FORCE:
