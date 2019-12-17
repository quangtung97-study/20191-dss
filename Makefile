.PHONY: all test format

all:
	cargo run

test:
	cargo test

format:
	python draw.py

count:
	git ls-files "*.rs" | xargs wc -l
