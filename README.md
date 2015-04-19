# picrust
An experiment in "bare metal" programming the Raspberry Pi using Rust

## Building on Linux
To compile you'll need the Raspberry Pi tools (https://github.com/raspberrypi/tools) and a Rust cross compiler for the Raspberry Pi. 
See https://github.com/npryce/rusty-pi/blob/master/doc/compile-the-compiler.asciidoc for instructions on how to compile the Rust compiler for this purpose.

The Makefile expects the Rapberry Pi tools in the ../pi-tools, and Rust in the ../pi-rust folder. With everything in place, simply call `make` to build everything.

## Running
To run on the Raspberry Pi, you'll first need the Raspberry Pi boot files. You can get the at https://github.com/raspberrypi/firmware/tree/master/boot. Take the `BOOTCODE.BIN`, `FIXUP.DAT` and `START.ELF` files and copy them to a FAT32 formatted SD card. Finally copy `kernel.img`, the result of the build, to the SD card, and boot your Raspberry Pi from it. Currently all it will do is turning one of the green leds on the Raspberry Pi on.
