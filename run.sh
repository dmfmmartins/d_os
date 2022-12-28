#!/bin/bash

printf "\nCleaning target directory and .bin files\n"
cargo clean
rm bios.bin
rm uefi.bin

printf "\nBuilding kernel\n"
cd kernel
cargo build --target x86_64-unknown-none

printf "\nBuilding .bin files and running QEMU\n"
cd ..
cargo run