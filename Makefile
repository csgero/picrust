RUSTDIR = ../pi-rust
RUSTC = $(RUSTDIR)/bin/rustc 
ARMGNU = ../pi-tools/arm-bcm2708/arm-bcm2708-linux-gnueabi/arm-bcm2708-linux-gnueabi/bin/
LD = ld 
BUILD = build/
export LD_LIBRARY_PATH = $(HOME)/rpi/pi-rust/lib

all: kernel.img kernel.list

rebuild: clean all

.SUFFIXES: .o .rs

.rs.o:
	$(RUSTC) -O --target=arm-unknown-linux-gnueabihf --crate-type lib -o $@ --emit obj $<

main.elf: kernel.ld main.o
	$(ARMGNU)/ld --no-undefined $(OBJECTS) -o $@ -T $^

kernel.img: main.elf
	$(ARMGNU)/objcopy $^ -O binary $@	

kernel.list: main.elf
	$(ARMGNU)/objdump -d $^ > $@

clean:
	-rm main.o
	-rm main.elf
	-rm kernel.img
	-rm kernel.list