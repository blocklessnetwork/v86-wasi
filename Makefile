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

default: target/v86-debug.wasm

release: target/v86.wasm

CARGO_FLAGS_SAFE=\
		--target wasm32-unknown-unknown \
		-- \
		-C linker="tools/rust-lld-wrapper" \
		-C link-args="--import-table --global-base=4096 $(STRIP_DEBUG_FLAG)" \
		-C link-args="target/softfloat.o" \
		-C link-args="target/zstddeclib.o" \
		--verbose

CARGO_FLAGS=$(CARGO_FLAGS_SAFE) -C target-feature=+bulk-memory   -C overflow-checks=off

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
	cargo run -p v86-wasi

clean:
	-rm target/libv86.js
	-rm target/libv86-debug.js
	-rm target/v86_all.js
	-rm target/v86.wasm
	-rm target/v86-debug.wasm
	-rm $(INSTRUCTION_TABLES)
	-rm target/*.map
	-rm target/*.wast
	-rm target/*.o
	$(MAKE) -C $(NASM_TEST_DIR) clean


tests: all-debug target/integration-test-fs/fs.json
	./tests/full/run.js

tests-release: all target/integration-test-fs/fs.json
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
	./tests/qemu/run.js > target/qemu-test-result
	./tests/qemu/run-qemu.js > target/qemu-test-reference
	diff target/qemu-test-result target/qemu-test-reference

qemutests-release: all
	$(MAKE) -C tests/qemu test-i386
	TEST_RELEASE_BUILD=1 time ./tests/qemu/run.js > target/qemu-test-result
	./tests/qemu/run-qemu.js > target/qemu-test-reference
	diff target/qemu-test-result target/qemu-test-reference

kvm-unit-test: all-debug
	(cd tests/kvm-unit-tests && ./configure && make x86/realmode.flat)
	tests/kvm-unit-tests/run.js tests/kvm-unit-tests/x86/realmode.flat

kvm-unit-test-release: all
	(cd tests/kvm-unit-tests && ./configure && make x86/realmode.flat)
	TEST_RELEASE_BUILD=1 tests/kvm-unit-tests/run.js tests/kvm-unit-tests/x86/realmode.flat

expect-tests: all-debug target/libwabt.js
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

