.PHONY: all

all: main.rs
	rustc -C opt-level=3 -C strip=symbols  main.rs && ./main

format:
	rustfmt main.rs
