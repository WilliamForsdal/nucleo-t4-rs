# Readme

Running `cargo build` does not actually build the correct binary.
We have to invoke `rustc` by doing `cargo rustc --release --target thumbv6m-none-eabi -- -C link-arg=-Tlink.x`
cortex-m-rt says we should also pass `-C link-arg=-nostartfiles` but that didn't work?
