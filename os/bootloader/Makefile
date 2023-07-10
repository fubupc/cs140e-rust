TARGET ?= aarch64-unknown-none

CARGO ?= cargo
CARGO_FLAGS ?= --target aarch64-unknown-none
OBJCOPY ?= rust-objcopy

RUST_BINARY := $(shell cat Cargo.toml | grep name | cut -d\" -f 2 | tr - _)
RUST_BUILD_DIR := target/$(TARGET)
RUST_DEBUG_BIN := $(RUST_BUILD_DIR)/debug/$(RUST_BINARY)
RUST_RELEASE_BIN := $(RUST_BUILD_DIR)/release/$(RUST_BINARY)

BUILD_DIR := build
KERNEL := $(BUILD_DIR)/$(RUST_BINARY).bin

.PHONY: all test clean check install $(RUST_DEBUG_BIN) $(RUST_RELEASE_BIN)

all: $(KERNEL)

check:
	@$(CARGO) check

test:
	@$(CARGO) test

$(RUST_DEBUG_BIN):
	@echo "+ Building $@ [cargo]"
	@$(CARGO) build $(CARGO_FLAGS)

$(RUST_RELEASE_BIN):
	@echo "+ Building $@ [cargo --release]"
	@$(CARGO) build $(CARGO_FLAGS) --release

ifeq ($(DEBUG),1)
$(KERNEL): $(RUST_DEBUG_BIN) | $(BUILD_DIR)
	@echo "+ Building $@ [objcopy $<]"
	@$(OBJCOPY) $< -O binary $@
else
$(KERNEL): $(RUST_RELEASE_BIN) | $(BUILD_DIR)
	@echo "+ Building $@ [objcopy $<]"
	@$(OBJCOPY) $< -O binary $@
endif

$(BUILD_DIR):
	@mkdir -p $@

clean:
	$(CARGO) clean
	rm -rf $(BUILD_DIR)
