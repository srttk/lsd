
install:
	cargo build --release
	cp ./target/release/lsd /usr/local/bin