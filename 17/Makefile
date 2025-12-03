.PHONY: all clean

PROBS = $(shell ls src/main*.c)
EXES = $(patsubst src/main%.c,build/%,$(PROBS))

all: $(EXES)

clean:
	rm -rf build

build/%: src/main%.c build
	gcc -o $@ $<

build:
	mkdir build
