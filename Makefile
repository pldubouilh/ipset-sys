build:
	cargo build

test:
	rm -rf target/debug/build/ipset-s*
	cargo test --no-run
	rm target/debug/deps/ipset_sys-*.d
	sudo target/debug/deps/ipset_sys-* -- tests

publish:
	# special char in ipset tests makes it impossible to publish directly with cargo :)
	rm -rf ipset/tests
	cargo publish
