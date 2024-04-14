# Echo

A simple operating system written in Rust called Echo. This is due to it only being made to output to a console much like the `echo` command because making a GUI is painful.

## Building

To build the project on Linux, run:
```
cargo rustc -- -Clink-arg=-nostartfiles
```

## License
Licensed under the GNU GPLv3 ([LICENSE](LICENSE) or https://www.gnu.org/licenses/gpl-3.0.en.html)
