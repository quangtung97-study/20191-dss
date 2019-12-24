.PHONY: all test format

all:
	cargo run

test:
	cargo test

format:
	cd example2 && python draw3d.py

count:
	git ls-files "*.rs" | xargs wc -l
