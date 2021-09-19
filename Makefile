COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)

.PHONY: all bench build check clean doc install run test repl

all: build

bench:
	$(CARGO) bench

build:
	$(CARGO) build

check:
	$(CARGO) check

clean:
	$(CARGO) clean

doc:
	$(CARGO) doc

install:
	$(CARGO) install

run:
	$(CARGO) run repl

test:
	$(CARGO) test -- --nocapture

repl:
	$(CARGO) run client -- repl