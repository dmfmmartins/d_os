use std::path::Path;

fn main() {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let uefi = false;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(Path::new("OVMF-pure-efi.fd"));
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }
    let child = cmd.spawn();
    child.ok();
}
