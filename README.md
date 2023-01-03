# Build Your Own OS

## Project

This is my journey writing an operational system from scratch with rust.

## How to Build

To build this binary, you need to compile for a bare metal target such as `thumbv7em-none-eabihf`:

```bash
$ rustup target add thumbv7em-none-eabihf
$ cargo build --target thumbv7em-none-eabihf
```

## References

[Writing an OS in Rust by Philipp Oppermann](https://os.phil-opp.com/)
