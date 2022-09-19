# Blockless WASI-v86 Extension

Blockless WASI-v86 Extension is an emulated v86 machine inside the Blockless Runtime Environment.


## How to build.

### 1. Build wasm 

use the follow command  to generate the wasm file. 
```bash
$ make
```

### 2. Run the test linux

use the follow command to run the linux of memory.

```bash
$ cargo run -p v86-wasi --release
```

