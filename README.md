# uefi-unifont
A small library to print Unicode string on UEFI's graphics output with [rust-osdev/uefi-rs](https://github.com/rust-osdev/uefi-rs).

`Cargo.toml`:
```toml
[dependencies]
uefi = { git = "https://github.com/rust-osdev/uefi-rs.git" }
uefi-unifont = { git = "https://github.com/mizkichan/uefi-unifont.git" }
```

`main.rs`:
```rust
use uefi::proto::console::gop::GraphicsOutput;
let gop: &mut GraphicsOutput = /* ... */;
uefi_unifont::print(gop, 0, 0, "Hello, world! こんにちは！");
```

![](screenshot.png)
