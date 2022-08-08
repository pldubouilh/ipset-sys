build::
	cargo build

run::
	rm -rf target/debug/ipset-sys
	cargo build
	sudo target/debug/ipset-sys

publish:: lint
	cargo publish

lint::
	cargo clippy --all
	cargo fmt --all

doc::
	cargo doc --open