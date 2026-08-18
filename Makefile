MODE ?= debug

CARGO_TARGET := x86_64-unknown-none

ifeq ($(MODE),release)
	CARGO_FLAGS := --release
	BUILD_DIR   := release
else
	CARGO_FLAGS :=
	BUILD_DIR   := debug
endif

.PHONY: kernel

kernel:
	cargo build --package brevyos --target $(CARGO_TARGET) $(CARGO_FLAGS)

test:
	cargo test --package brevyos --target $(CARGO_TARGET)

clean:
	cargo clean
