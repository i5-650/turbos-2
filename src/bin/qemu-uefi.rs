use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::{
    any::Any,
    env,
    process::{self, Command},
};

// todo: fix the not found error
fn main() {
    let prebuilt =
        Prebuilt::fetch(Source::LATEST, "target/ovmf").expect("failed to update prebuilt");
    println!("{:?}", prebuilt.type_id());
    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-drive");
    qemu.arg(format!("format=raw,file={}", env!("UEFI_IMAGE")));
    qemu.arg("-bios")
        .arg(prebuilt.get_file(Arch::X64, FileType::Code));
    let exit_status = qemu.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}
