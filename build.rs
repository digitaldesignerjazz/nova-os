// build.rs — Bootloader & Kernel Build Configuration for Nova OS

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Recommended linker settings for kernel
    println!("cargo:rustc-link-arg=-zmax-page-size=0x1000");

    // Note for developers:
    // To build a bootable image, you can use:
    //
    //   cargo bootimage          (legacy but still works with bootloader = "0.11")
    //
    // Or the more modern approach:
    //   Add a small binary that depends on `bootloader` crate + this kernel.
    //
    // See docs/roadmap.md for current status and next steps.
}
