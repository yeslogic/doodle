CC = gcc
CFLAGS = -O2
TGT_DIR=..

%.o: %.c
	$(CC) -c -o $@ $< $(CFLAGS)

all: test.elf

test.elf: elf.o
	$(CC) -o $(TGT_DIR)/test.elf elf.o

clean:
	rm elf.o
