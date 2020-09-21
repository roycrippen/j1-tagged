all: j1 j1.bin j1.hex hex2rust

hex2rust:
	python3 hex2rust.py
j1: j1.c
	gcc -o j1 j1.c
j1.bin j1.hex: j1.4th
	gforth ./j1.4th
run: all
	./j1
core: all
	./j1 core.4th
clean:
	rm -rf j1 j1.bin j1.hex j1_bytes.rs
