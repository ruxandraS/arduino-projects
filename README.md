# arduino-projects

Arduino starter kit projects

# setup rust env on Mac OS [logrocket guide][logrocket]

- Install cargo: https://rustup.rs
- Install brew: https://brew.sh
- Set HOME Path `%sudo chown -R <OWNER> /opt/homebrew`

- `%rustup toolchain install nightly`
- `%brew tap osx-cross/avr`
- `%brew install avr-binutils avr-gcc avrdude`
- `%cargo install cargo-generate`
- `%cargo +stable install ravedude`

# setup project

- `%cargo generate --git https://github.com/Rahix/avr-hal-template.git`
- enter project name

# run project
- Identify via Arduino IDE or
`%ls /dev/cu.usbmodem.*`
- `%export RAVEDUDE_PORT=<NAME>`
- `%cargo run`

[logrocket]: https://blog.logrocket.com/complete-guide-running-rust-arduino/#on-macos