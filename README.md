# Echo

A simple operating system written in Rust called Echo. This is due to it only being made to output to a console much like the `echo` command because making a GUI is painful.

## Building

You can build the project by running:
```
cargo build
```
To create a bootable disk image from the compiled kernel, you need to install the [`bootimage`] tool:

[`bootimage`]: https://github.com/rust-osdev/bootimage
```
cargo install bootimage
```

After installing, you can create the bootable disk image by running:
```
cargo bootimage
```

This creates a bootable disk image in the `target/x86_64-echo/debug` directory.

## Running
You can run the disk image in [QEMU] through:

[QEMU]: https://www.qemu.org/
```
cargo run
```

[QEMU] and the [`bootimage`] tool need to be installed for this.
You can also write the image to a USB stick for booting it on a real machine. On Linux, the command for this is:
```
dd if=target/x86_64-echo/debug/bootimage-echo.bin of=/dev/sdX && sync
```

Where `sdX` is the device name of your USB stick. **Be careful** to choose the correct device name, because everything on that device is overwritten.

## License
Licensed under the GNU GPLv3 ([LICENSE](LICENSE) or https://www.gnu.org/licenses/gpl-3.0.en.html)
