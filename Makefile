NASM_TEST_DIR=./tests/nasm

INSTRUCTION_TABLES=crates/v86/src/gen/jit.rs crates/v86/src/gen/jit0f.rs \
		   crates/v86/src/gen/interpreter.rs crates/v86/src/gen/interpreter0f.rs \
		   crates/v86/src/gen/analyzer.rs crates/v86/src/gen/analyzer0f.rs \

# Only the dependencies common to both generate_{jit,interpreter}.js
GEN_DEPENDENCIES=$(filter-out crates/v86/gen/generate_interpreter.js crates/v86/gen/generate_jit.js crates/v86/gen/generate_analyzer.js, $(wildcard crates/v86/gen/*.js))
JIT_DEPENDENCIES=$(GEN_DEPENDENCIES) crates/v86/gen/generate_jit.js
INTERPRETER_DEPENDENCIES=$(GEN_DEPENDENCIES) crates/v86/gen/generate_interpreter.js
ANALYZER_DEPENDENCIES=$(GEN_DEPENDENCIES) crates/v86/gen/generate_analyzer.js

V86_TARGET_PATH=../../target

STRIP_DEBUG_FLAG=
ifeq ($(STRIP_DEBUG),true)
STRIP_DEBUG_FLAG=--v86-strip-debug
endif

WASM_OPT ?= false

default: build release

debug: target/v86-debug.wasm 

release: target/v86.wasm

CARGO_FLAGS_SAFE=\
		--target wasm32-unknown-unknown \
		-- \
		-C linker="tools/rust-lld-wrapper" \
		-C link-args="--import-table --global-base=4096 $(STRIP_DEBUG_FLAG)" \
		-C link-args="target/softfloat.o" \
		-C link-args="target/zstddeclib.o" \
		--verbose

CARGO_FLAGS=$(CARGO_FLAGS_SAFE) -C target-feature=+bulk-memory -C target-feature=+multivalue -C target-feature=+simd128

RUST_FILES=$(shell find crates/v86/src/ -name '*.rs') \
	   crates/v86/src/gen/interpreter.rs crates/v86/src/gen/interpreter0f.rs \
	   crates/v86/src/gen/jit.rs crates/v86/src/gen/jit0f.rs \
	   crates/v86/src/gen/analyzer.rs crates/v86/src/gen/analyzer0f.rs

crates/v86/src/gen/jit.rs: $(JIT_DEPENDENCIES)
	crates/v86/gen/generate_jit.js --output-dir target/ --table jit
crates/v86/src/gen/jit0f.rs: $(JIT_DEPENDENCIES)
	crates/v86/gen/generate_jit.js --output-dir target/ --table jit0f

crates/v86/src/gen/interpreter.rs: $(INTERPRETER_DEPENDENCIES)
	crates/v86/gen/generate_interpreter.js --output-dir target/ --table interpreter
crates/v86/src/gen/interpreter0f.rs: $(INTERPRETER_DEPENDENCIES)
	crates/v86/gen/generate_interpreter.js --output-dir target/ --table interpreter0f

crates/v86/src/gen/analyzer.rs: $(ANALYZER_DEPENDENCIES)
	crates/v86/gen/generate_analyzer.js --output-dir target/ --table analyzer
crates/v86/src/gen/analyzer0f.rs: $(ANALYZER_DEPENDENCIES)
	crates/v86/gen/generate_analyzer.js --output-dir target/ --table analyzer0f

target/v86.wasm: $(RUST_FILES) target/softfloat.o target/zstddeclib.o Cargo.toml
	mkdir -p target/
	-BLOCK_SIZE=K ls -l target/v86.wasm
	cd  crates/v86 && CARGO_TARGET_PATH=$(V86_TARGET_PATH) cargo rustc --release $(CARGO_FLAGS) 
	mv target/wasm32-unknown-unknown/release/v86.wasm target/v86.wasm
	-$(WASM_OPT) && wasm-opt -O2 --strip-debug target/v86.wasm -o target/v86.wasm
	BLOCK_SIZE=K ls -l target/v86.wasm

target/v86-debug.wasm: $(RUST_FILES) target/softfloat.o target/zstddeclib.o Cargo.toml
	mkdir -p target/
	-BLOCK_SIZE=K ls -l target/v86-debug.wasm
	cd  crates/v86 && CARGO_TARGET_PATH=$(V86_TARGET_PATH) cargo rustc $(CARGO_FLAGS) 
	mv target/wasm32-unknown-unknown/debug/v86.wasm target/v86-debug.wasm
	BLOCK_SIZE=K ls -l target/v86-debug.wasm

target/v86-fallback.wasm: $(RUST_FILES) target/softfloat.o target/zstddeclib.o Cargo.toml
	mkdir -p target/
	cd  crates/v86 && CARGO_TARGET_PATH=$(V86_TARGET_PATH) cargo rustc --release $(CARGO_FLAGS_SAFE)
	mv target/wasm32-unknown-unknown/release/v86.wasm target/v86-fallback.wasm || true

debug-with-profiler: $(RUST_FILES) target/softfloat.o target/zstddeclib.o Cargo.toml
	mkdir -p target/
	cargo rustc --features profiler $(CARGO_FLAGS)
	mv target/wasm32-unknown-unknown/debug/v86.wasm target/v86-debug.wasm || true


with-profiler: $(RUST_FILES) target/softfloat.o target/zstddeclib.o Cargo.toml
	mkdir -p target/
	cargo rustc --release --features profiler $(CARGO_FLAGS)
	mv target/wasm32-unknown-unknown/release/v86.wasm target/v86.wasm || true

target/softfloat.o: lib/softfloat/softfloat.c
	mkdir -p target
	clang -c -Wall \
	    --target=wasm32 -O3 -flto -nostdlib -fvisibility=hidden -ffunction-sections -fdata-sections \
	    -DSOFTFLOAT_FAST_INT64 -DINLINE_LEVEL=5 -DSOFTFLOAT_FAST_DIV32TO16 -DSOFTFLOAT_FAST_DIV64TO32 \
	    -o target/softfloat.o \
	    lib/softfloat/softfloat.c

target/zstddeclib.o: lib/zstd/zstddeclib.c
	mkdir -p target
	clang -c -Wall \
	    --target=wasm32 -O3 -flto -nostdlib -fvisibility=hidden -ffunction-sections -fdata-sections \
	    -DZSTDLIB_VISIBILITY="" \
	    -o target/zstddeclib.o \
	    lib/zstd/zstddeclib.c

run_debug:
	cargo run -p v86-wasi  boot.json

run:
	cargo run --release -p v86-wasi  boot.json

build:
	cargo build --release -p v86-wasi

build_debug:
	cargo build  -p v86-wasi

clean:
	-rm target/v86.wasm
	-rm target/v86-debug.wasm
	-rm $(INSTRUCTION_TABLES)
	-rm target/*.o
	cargo clean




rustfmt: $(RUST_FILES)
	cargo fmt --all -- --check

