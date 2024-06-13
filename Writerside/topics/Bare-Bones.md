# Bare Bones

## VGA Text Mode
The [VGA text mode](https://en.wikipedia.org/wiki/VGA_text_mode) is a simple way to print text to the screen.

### The VGA Text Buffer
To print a character to the screen in VGA text mode, one has to write it to the text buffer of the VGA hardware. The VGA
text buffer is a two-dimensional array with typically 25 rows and 80 columns, which is directly rendered to the screen.
Each array entry describes a single screen character though the following format:

| Bit(s) | Value            |
|--------|------------------|
| 0-7    | ASCII code point |
| 8-11   | Foreground color |
| 12-14  | Background color |
| 15     | Blink            |            

The first byte represents the character that should be printed in the 
[ASCII encoding](https://en.wikipedia.org/wiki/ASCII).
To be more specific, it isn't exactly ASCII, but a character set named 
[code page 437](https://en.wikipedia.org/wiki/Code_page_437) with some additional characters and slight modifications.
However, for the rest of this page I will refer to it as an ASCII character in this post.

The second byte defines how the character is displayed. The first four bits define the foreground color, the next three
bits the background color, and the last bit whether the character should blink.
The following colors are available:

| Number | Color      | Number + Bright Bit | Bright Color |
|--------|------------|---------------------|--------------|
| 0x0    | Black      | 0x8                 | Dark Gray    |
| 0x1    | Blue       | 0x9                 | Light Blue   |
| 0x2    | Green      | 0xa                 | Light Green  |
| 0x3    | Cyan       | 0xb                 | Light Cyan   |
| 0x4    | Red        | 0xc                 | Light Red    |
| 0x5    | Magenta    | 0xd                 | Pink         |
| 0x6    | Brown      | 0xe                 | Yellow       |
| 0x7    | Light Gray | 0xf                 | White        |

Bit 4 is the *bright bit*, which turns, for example, blue into light blue. For the background color, this bit is 
repurposed as the blink bit.

The VGA text buffer is accessible via [memory-mapped I/O](https://en.wikipedia.org/wiki/Memory-mapped_I/O) to address
`0xb80000`. This means that reads and writes to that address don't access the RAM but directly access the text buffer on
the VGA hardware. This means we can read and write it through normal memory operations to that address.

Note that memory-mapped hardware might not support all normal RAM operations. For example, a device could only support
byte-wise reads and return junk when a `u64` is read. Fortunately, the text buffer 
[supports normal reads and writes](https://web.stanford.edu/class/cs140/projects/pintos/specs/freevga/vga/vgamem.htm#manip),
so we don't have to treat it in a special way.
