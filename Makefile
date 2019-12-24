build:
	cargo build --release

install:
	cp target/release/qt_thread_eg ~/bin/qtThreadEg

all: build install