rust_sfml_path = rust-sfml
rust_sfml_lib = $(rust_sfml_path)/lib/

.SILENT:

all: build

rust_sfml_build:
	cd rust_sfml_path
	make lib

build:
	mkdir -p bin
	rustc --out-dir bin/ -L $(rust_sfml_lib) src/main.rs 

clean:
	rm -rf bin
