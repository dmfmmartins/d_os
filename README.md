# d_os

Trying to create an OS in RUST

## Requirements

- QEMU (x86_64)
- Rust nightly
- Rust "dependencies":
  - ```sh
    rustup component add llvm-tools-preview
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
    ```
- For Ubuntu (22.10):
  - ```sh
    sudo apt install build-essential
    ```

## Compiling and running

Assuming you are on the project root directory `d_os/`
```sh
# Compile kernel
cd kernel
cargo build --target x86_64-unknown-none

# Create `bios.bin` and `uefi.bin`
cd ..
cargo build

# OR

# Create `bios.bin` and `uefi.bin` and opens QEMU with correct settings
cd ..
cargo run
```

## Current Goals

- [ ] VGA Graphics

### More info and inspiration

- [Bootloader (crate)](https://github.com/rust-osdev/bootloader)
- [BlogOS](https://os.phil-opp.com/)
- [Redox](https://www.redox-os.org/)
- [Stupid Operating System (SOS)](https://github.com/sos-os/kernel)
- [OS Dev (Rust)](https://wiki.osdev.org/Rust)
