# About Echo

Echo is a simple operating system built with Rust.

## Setup Guide

This guide will walk you through setting up, building, and running Echo in a QEMU VM on your local machine.

> **Windows Users**
>
> It may be difficult to use QEMU on Windows. It is recommended that you use [WSL](https://learn.microsoft.com/en-us/windows/wsl/about)
> or run the project in a virtual machine using [VMWare Workstation Player](https://www.vmware.com/products/workstation-player.html) instead.
>
{style="note"}

### Before you start

Make sure that:
- [Rust](https://www.rust-lang.org/tools/install) is installed on your machine
- [QEMU](https://www.qemu.org/download/) is installed on your machine if you want to virtualize Echo

### Building the project

Build the project by running:
```bash
cargo build
```

This will create a bootable disk image in `target/debug/build/echo-XXXXXXXXXXXXXXXX/out/`.\
This directory will contain either one or two disk images depending on uefi or bios boot.

### Running
You can run the disk image in QEMU through
```bash
cargo run
```
You can alternatively write the image to a USB stick for booting it on a real machine. On Linux, the command for this is:
```bash
dd if=target/x86_64-echo/debug/bootimage-echo.bin of=/dev/sdX && sync
```
Where sdX is the device name of your USB stick. **Be careful** to choose the correct device name, because everything on that device is overwritten.

## License
This project is licensed under [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html)
