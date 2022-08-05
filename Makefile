export PKG_CONFIG_PATH=./ipset/outlib/lib

build::
	cargo build

build-deps::
	mkdir -p outlib
	cd ipset &&\
		./autogen.sh &&\
		./configure --prefix=/ --sbindir=/ --with-kmod=no &&\
		make &&\
		make DESTDIR=${PWD}/outlib install

test::
	cargo test --no-run
	rm -rf target/debug/deps/ipset_sys-*.d
	sudo LD_PRELOAD=outlib/lib/libipset.so target/debug/deps/ipset_sys-* -- tests

publish:: lint
	cargo publish

lint::
	cargo clippy --all
	cargo fmt --all
