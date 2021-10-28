# `stm32f4-md407`

Board support crate for [md407](md407) board

# Dependencies

To build embedded programs using this you'll need:

- Rust 1.51 or newer toolchain
- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  target.

```console
$ cargo install cargo-generate
$ rustup target add thumbv7em-none-eabi thumbv7em-none-eabihf
```

For more info on working with embedded Rust, see the [Embedded Rust Book][book].

<!-- references -->

[md407]: http://www.cse.chalmers.se/edu/resources/mop/documents/MD407_beskrivning.pdf
[book]: https://rust-embedded.github.io/book
