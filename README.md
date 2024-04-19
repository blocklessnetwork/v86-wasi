# Blockless WASI-v86 Extension

Blockless WASI-v86 Extension is an emulated v86 machine inside the Blockless
Runtime Environment.

## How to build.

### 1. Build wasm

install `wasm-unknown-unknown` target

```bash
$ rustup target add wasm32-unknown-unknown
```

use the follow command to generate the wasm file.

```bash
$ make release
```

### 2. Modify the config file.

The follow is the configure file

```json
{
  "cdrom": "arch/linux4.iso",
  "bios_file": "arch/seabios.bin",
  "vga_bios_file": "arch/vgabios.bin",
  "wasm_file": "target/v86.wasm",
  "memory_size": 134217728,
  "vga_memory_size": 8388608,
  "cmdline": ["tsc=reliable mitigations=off random.trust_cpu=on"],
  "logger": {
    "log_file": "debug.log",
    "log_module": ["E", "BIOS", "NET"]
  },
  "tun": {
    "address": "192.168.0.1",
    "netmask": "255.255.255.0",
    "ether_address": "00:22:15:fe:ae:ba"
  },
  "muiltiboot_order": ["bin", "cdrom"]
}
```

### 3. Run the test linux

use the following command to run the linux with the configure file.

```bash
$ cargo run -p v86-wasi --release ./boot.json
```

After run the VM, you can open the "term.html" file for control the VM.

![](term/Screen.png)

### 4. DIY the linux iso

If you wanna DIY the linux by your self, please see the document
"[https://github.com/blocklessnetwork/build-blockless-linux](https://github.com/blocklessnetwork/build-blockless-linux)"

## v86 Lib compilation

```sh
cargo build --release -p v86-lib
```

This will generate a `libv86_lib.dylib` file for your machine arch (which can be packaged in the car format to be run in the runtime)

