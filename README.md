Using Rust to blink an LED on an embedded device

This example is mainly copied from the great Rust Embedded [Cortex-M Quickstart](https://github.com/rust-embedded/cortex-m-quickstart)
and Jesse Braham's article [Embedded Rust: From Zero to Blinky](https://beta7.io/posts/embedded-rust-from-zero-to-blinky.html)
and tweaked for the STM32F446RE of the NUCLEO-F446RE board.

prerequisites: rust, opencd

0. `rustup target add thumbv7em-none-eabihf`

1. `git clone https://github.com/krenzlin/rust-stm32f466-blinky`
2. `cd rust-stm32f466-blinky`
3. `cargo build`
4. `sudo ./flash_device.sh target/thumbv7em-none-eabihf/debug/blinky`
