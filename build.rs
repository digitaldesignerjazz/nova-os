// build.rs - Bootloader configuration for Nova OS

fn main() {
    // Tell cargo to rerun this script if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");

    // Configure bootloader (modern bootloader crate v0.11+)
    // This enables physical memory mapping which is useful for later kernel development
    println!("cargo:rustc-link-arg=-zmax-page-size=0x1000");

    // Note: For full bootable image generation, use:
    //   cargo bootimage (legacy but still works)
    // or the modern approach with the bootloader crate as a dependency
    // See docs/roadmap.md for current recommended steps
}
