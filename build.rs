use std::path::Path;

fn main() {
    let kernel = Path::new("kernel/src/main.rs");

    // create an UEFI disk image (optional)
    let uefi_path = Path::new("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(uefi_path)
        .unwrap();

    // create a BIOS disk image (optional)
    let bios_path = Path::new("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(bios_path)
        .unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
