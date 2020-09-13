# ilum
Ilum is a collection of self-written crates to provide support for the Rust programming language on the Teensy 4.0

## Dependencies
- A Rust installation, I recommend an installation using `rustup`
- The `thumbv7em-none-eabihf` Rust target, which may be installed using `rustup`:

    `$ rustup target add thumbv7em-none-eabihf`

- `objcopy` for transforming Rust binaries into hex files. I use LLVM objcopy provided by
    
    [`cargo binutils`](https://github.com/rust-embedded/cargo-binutils)
- Last but not least you'll need the [teensy_loader_cli](https://github.com/PaulStoffregen/teensy_loader_cli) or [Teensy Loader Application](https://www.pjrc.com/teensy/loader.html)

    to download your programs to your Teensy 4.0.

## Usage
Coming soon

## Project structure
This project includes several crates. 
- `ilum-init` is setting up the MCU for use.
- `ilum-hal` is a Hardware-Access-Layer for the iMXRT1062.
- `ilum-bsp` is the board-support-package (coming) which depends on the depends on the previous mentioned crates.
    
### ilum-init
Coming soon

### ilum-hal
Coming soon


## Contributing
Contributions to code and documentation are heavily appreciated, may it be a bug fix, a new feature, or improvement of the code or wiki documentation.

## Support
If something isn't working as expected or you have questions, feel free to open an [issue](https://github.com/Lockna/ilum/issues/new).

If you are looking for a more direct way to contact me, you are welcome to send me an [e-mail](mailto:raphael.ob@protonmail.com)

## License
Ilum is distributed under the terms of either Apache License Version 2.0 or the MIT license, at the user's choice. See [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE)