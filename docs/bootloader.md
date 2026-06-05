# Building a Bootable Nova OS Image

## Recommended Approach (Modern)

The cleanest way is to create a small bootloader binary that depends on this kernel.

### Structure

```
nova-os/
├── Cargo.toml          # kernel crate
├── src/                # kernel code
└── boot/               # bootloader binary (recommended)
    ├── Cargo.toml
    └── src/main.rs
```

### boot/Cargo.toml

```toml
[package]
name = "nova-os-boot"
version = "0.1.0"
edition = "2021"

[dependencies]
bootloader = { version = "0.11", features = ["map_physical_memory"] }
nova-os = { path = ".." }
```

### boot/src/main.rs

```rust
#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Initialize Nova OS kernel here
    nova_os::init(boot_info);

    loop {}
}
```

## Alternative: cargo bootimage (Legacy but simple)

You can still use:

```bash
cargo bootimage
```

This produces `target/x86_64-unknown-none/debug/bootimage-nova-os.bin`

## Current Status

- `build.rs` is configured
- `bootloader` dependency is present
- Full bootloader binary crate is recommended for clean separation

See `docs/roadmap.md` for the current development phase.
