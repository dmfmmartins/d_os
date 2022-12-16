# d_os

Trying to create an OS

## Requirements

- QEMU (x86_64)
- Rust nightly
- Rust "dependencies":
  - ```sh
      rustup target add x86_64-unknown-none
      rustup component add llvm-tools-preview
    ```

## Compiling and running

```rust
// Create `bios.img` and `uefi.img`
cargo build

// Create `bios.img` and `uefi.img` and opens QEMU with correct settings
cargo run
```

## Goals

- [ ] Actually being able to boot into the kernel and not hang up with the error "Did not find ELF magic number"

### More info and inspiration

- [Bootloader (crate)](https://github.com/rust-osdev/bootloader)
- [BlogOS](https://os.phil-opp.com/)
- [Redox](https://www.redox-os.org/)
- [Stupid Operating System (SOS)](https://github.com/sos-os/kernel)
- [OS Dev (Rust)](https://wiki.osdev.org/Rust)
