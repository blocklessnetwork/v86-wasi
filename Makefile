NASM_TEST_DIR=./tests/nasm

TARGET_DIR=build/

INSTRUCTION_TABLES=crates/v86/src/gen/jit.rs crates/v86/src/gen/jit0f.rs \
		   crates/v86/src/gen/interpreter.rs crates/v86/src/gen/interpreter0f.rs \
		   crates/v86/src/gen/analyzer.rs crates/v86/src/gen/analyzer0f.rs \

# Only the dependencies common to both generate_{jit,interpreter}.js
GEN_DEPENDENCIES=$(filter-out crates/gen/generate_interpreter.js crates/gen/generate_jit.js crates/gen/generate_analyzer.js, $(wildcard crates/gen/*.js))
JIT_DEPENDENCIES=$(GEN_DEPENDENCIES) crates/gen/generate_jit.js
INTERPRETER_DEPENDENCIES=$(GEN_DEPENDENCIES) crates/gen/generate_interpreter.js
ANALYZER_DEPENDENCIES=$(GEN_DEPENDENCIES) crates/gen/generate_analyzer.js

STRIP_DEBUG_FLAG=
ifeq ($(STRIP_DEBUG),true)
STRIP_DEBUG_FLAG=--v86-strip-debug
endif

WASM_OPT ?= false

default: build/v86-debug.wasm

CARGO_FLAGS_SAFE=\
		--target wasm32-wasi \
		-- \
		-C linker=tools/rust-lld-wrapper \
		-C link-args="--global-base=4096 $(STRIP_DEBUG_FLAG)" \
		-C link-args="build/softfloat.o" \
		-C link-args="build/zstddeclib.o" \
		--verbose

CARGO_FLAGS=$(CARGO_FLAGS_SAFE) -C target-feature=+bulk-memory 

RUST_FILES=$(shell find crates/v86/src/ -name '*.rs') \
	   crates/v86/src/gen/interpreter.rs crates/v86/src/gen/interpreter0f.rs \
	   crates/v86/src/gen/jit.rs crates/v86/src/gen/jit0f.rs \
	   crates/v86/src/gen/analyzer.rs crates/v86/src/gen/analyzer0f.rs

crates/v86/src/gen/jit.rs: $(JIT_DEPENDENCIES)
	crates/gen/generate_jit.js --output-dir build/ --table jit
crates/v86/src/gen/jit0f.rs: $(JIT_DEPENDENCIES)
	crates/gen/generate_jit.js --output-dir build/ --table jit0f

crates/v86/src/gen/interpreter.rs: $(INTERPRETER_DEPENDENCIES)
	crates/gen/generate_interpreter.js --output-dir build/ --table interpreter
crates/v86/src/gen/interpreter0f.rs: $(INTERPRETER_DEPENDENCIES)
	crates/gen/generate_interpreter.js --output-dir build/ --table interpreter0f

crates/v86/src/gen/analyzer.rs: $(ANALYZER_DEPENDENCIES)
	crates/gen/generate_analyzer.js --output-dir build/ --table analyzer
crates/v86/src/gen/analyzer0f.rs: $(ANALYZER_DEPENDENCIES)
	crates/gen/generate_analyzer.js --output-dir build/ --table analyzer0f

build/v86.wasm: $(RUST_FILES) build/softfloat.o build/zstddeclib.o Cargo.toml
	mkdir -p build/
	-BLOCK_SIZE=K ls -l build/v86.wasm
	CARGO_TARGET_DIR=$(TARGET_DIR) cargo rustc --release $(CARGO_FLAGS) 
	mv build/wasm32-wasi/release/v86.wasm build/v86.wasm
	-$(WASM_OPT) && wasm-opt -O2 --strip-debug build/v86.wasm -o build/v86.wasm
	BLOCK_SIZE=K ls -l build/v86.wasm

build/v86-debug.wasm: $(RUST_FILES) build/softfloat.o build/zstddeclib.o Cargo.toml
	mkdir -p build/
	-BLOCK_SIZE=K ls -l build/v86-debug.wasm
	CARGO_TARGET_DIR=$(TARGET_DIR) cargo  rustc $(CARGO_FLAGS)
	mv build/wasm32-wasi/debug/v86.wasm build/v86-debug.wasm
	BLOCK_SIZE=K ls -l build/v86-debug.wasm

build/v86-fallback.wasm: $(RUST_FILES) build/softfloat.o build/zstddeclib.o Cargo.toml
	mkdir -p build/
	CARGO_TARGET_DIR=$(TARGET_DIR) cargo rustc --release $(CARGO_FLAGS_SAFE)
	mv build/wasm32-wasi/release/v86.wasm build/v86-fallback.wasm || true

debug-with-profiler: $(RUST_FILES) build/softfloat.o build/zstddeclib.o Cargo.toml
	mkdir -p build/
	CARGO_TARGET_DIR=$(TARGET_DIR) cargo rustc --features profiler $(CARGO_FLAGS)
	mv build/wasm32-wasi/debug/v86.wasm build/v86-debug.wasm || true

with-profiler: $(RUST_FILES) build/softfloat.o build/zstddeclib.o Cargo.toml
	mkdir -p build/
	CARGO_TARGET_DIR=$(TARGET_DIR) cargo rustc --release --features profiler $(CARGO_FLAGS)
	mv build/wasm32-wasi/release/v86.wasm build/v86.wasm || true

build/softfloat.o: lib/softfloat/softfloat.c
	mkdir -p build
	clang -c -Wall \
	    --target=wasm32 -O3 -flto -nostdlib -fvisibility=hidden -ffunction-sections -fdata-sections \
	    -DSOFTFLOAT_FAST_INT64 -DINLINE_LEVEL=5 -DSOFTFLOAT_FAST_DIV32TO16 -DSOFTFLOAT_FAST_DIV64TO32 \
	    -o build/softfloat.o \
	    lib/softfloat/softfloat.c

build/zstddeclib.o: lib/zstd/zstddeclib.c
	mkdir -p build
	clang -c -Wall \
	    --target=wasm32 -O3 -flto -nostdlib -fvisibility=hidden -ffunction-sections -fdata-sections \
	    -DZSTDLIB_VISIBILITY="" \
	    -o build/zstddeclib.o \
	    lib/zstd/zstddeclib.c

clean:
	-rm build/libv86.js
	-rm build/libv86-debug.js
	-rm build/v86_all.js
	-rm build/v86.wasm
	-rm build/v86-debug.wasm
	-rm $(INSTRUCTION_TABLES)
	-rm build/*.map
	-rm build/*.wast
	-rm build/*.o
	$(MAKE) -C $(NASM_TEST_DIR) clean




tests: all-debug build/integration-test-fs/fs.json
	./tests/full/run.js

tests-release: all build/integration-test-fs/fs.json
	TEST_RELEASE_BUILD=1 ./tests/full/run.js

nasmtests: all-debug
	$(MAKE) -C $(NASM_TEST_DIR) all
	$(NASM_TEST_DIR)/gen_fixtures.js
	$(NASM_TEST_DIR)/run.js

nasmtests-force-jit: all-debug
	$(MAKE) -C $(NASM_TEST_DIR) all
	$(NASM_TEST_DIR)/gen_fixtures.js
	$(NASM_TEST_DIR)/run.js --force-jit

jitpagingtests: all-debug
	$(MAKE) -C tests/jit-paging test-jit
	./tests/jit-paging/run.js

qemutests: all-debug
	$(MAKE) -C tests/qemu test-i386
	./tests/qemu/run.js > build/qemu-test-result
	./tests/qemu/run-qemu.js > build/qemu-test-reference
	diff build/qemu-test-result build/qemu-test-reference

qemutests-release: all
	$(MAKE) -C tests/qemu test-i386
	TEST_RELEASE_BUILD=1 time ./tests/qemu/run.js > build/qemu-test-result
	./tests/qemu/run-qemu.js > build/qemu-test-reference
	diff build/qemu-test-result build/qemu-test-reference

kvm-unit-test: all-debug
	(cd tests/kvm-unit-tests && ./configure && make x86/realmode.flat)
	tests/kvm-unit-tests/run.js tests/kvm-unit-tests/x86/realmode.flat

kvm-unit-test-release: all
	(cd tests/kvm-unit-tests && ./configure && make x86/realmode.flat)
	TEST_RELEASE_BUILD=1 tests/kvm-unit-tests/run.js tests/kvm-unit-tests/x86/realmode.flat

expect-tests: all-debug build/libwabt.js
	make -C tests/expect/tests
	./tests/expect/run.js

devices-test: all-debug
	./tests/devices/virtio_9p.js

rust-test: $(RUST_FILES)
	env RUSTFLAGS="-D warnings" RUST_BACKTRACE=full RUST_TEST_THREADS=1 cargo test -- --nocapture
	./tests/rust/verify-wasmgen-dummy-output.js

rust-test-intensive:
	QUICKCHECK_TESTS=100000000 make rust-test

api-tests: all-debug
	./tests/api/clean-shutdown.js
	./tests/api/state.js
	./tests/api/reset.js

all-tests: jshint kvm-unit-test qemutests qemutests-release jitpagingtests api-tests nasmtests nasmtests-force-jit tests expect-tests
	# Skipping:
	# - devices-test (hangs)

jshint:
	jshint --config=./.jshint.json src tests gen lib

rustfmt: $(RUST_FILES)
	cargo fmt --all -- --check

