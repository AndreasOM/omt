r:
	cargo run --release

d:
	cargo run

cr:
	cargo check --release

c:
	cargo check
	
fmt:
	cargo +nightly fmt

test-noisy:
	cargo test -- --nocapture

release-alpha:
	omr-bumper -b patch -r alpha

