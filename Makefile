.SILENT:

all:
	mkdir -p bin
	rustc --out-dir bin main.rs

clean:
	rm -rf bin
