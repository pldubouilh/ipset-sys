build::
	cargo build

test::
	cargo test --no-run
	rm target/debug/deps/ipset_sys-*.d
	sudo target/debug/deps/ipset_sys-* -- tests

publish:: lint
	cargo publish

lint::
	cargo clippy --all
	cargo fmt --all
