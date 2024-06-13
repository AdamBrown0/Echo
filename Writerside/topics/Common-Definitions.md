# Common Definitions

This page will house some of the common definitions used across the project.

### Lazy Statics
The one-time initialization of statics with non-const functions is a common problem in Rust.\
Fortunately, there already exists a good solution a crate named
[lazy_static](https://docs.rs/lazy_static/1.0.1/lazy_static/).
This crate provides a `lazy_static!` macro that defines a lazily initialized `static`. Instead of computing its value at
compile time, the `static` lazily initializes itself when accessed for the first time. Thus, the initialization happens
at runtime, so arbitrarily complex initialization code is possible.

For an example of using the `lazy_static` crate, here we can define our static [VGA](Bare-Bones.md#vga-text-mode)
`WRITER` without problems:
```text
// in src/vga.rs

use lazy_static::lazy_static;

lazy_static! {
    pub static ref WRITER: Writer = {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb80000 as *mut Buffer) },
    };
}
```

### Spinlocks
To get synchronized interior mutability, users of the standard library can use 
[Mutex](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html). It provides mutual exclusion by blocking threads
when the resource is already locked. But our basic kernel does not have any blocking support or even a concept of 
threads, so we can't use it either. However, there is a really basic kind of mutex in computer science that requires no
operating system features: the [spinlock](https://en.wikipedia.org/wiki/Spinlock). Instead of blocking, the threads
simply try to lock it again and again in a tight loop, thus burning CPU time until the mutex is free again.

Once again we are going to use the example of our static [VGA](Bare-Bones.md#the-vga-text-buffer) `WRITER`:
```text
// in src/vga.rs

use spin::Mutex;
...
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
```

