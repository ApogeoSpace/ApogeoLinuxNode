BUILD_DIRECTORY := $(DIR)/build
KERNEL_DIR := /lib/modules/$(shell uname -r)/build
KERNEL_MODULE := iotd
MODULE_SOURCES := $(wildcard src/*.c)
RUST_SOURCES := $(wildcard $(TOPDIR)/iotd/src/*.rs)
BUILD_TYPE := debug

module/include/iotd/rs/provided.h:
	@cd "$(TOPDIR)" && cbindgen --config iotd/cbindgen.toml --crate iotd --output module/include/iotd/rs/provided.h

all modules iotd: $(BUILD_DIRECTORY)/Makefile module/include/iotd/rs/provided.h
	MODULE_SOURCES="$(MODULE_SOURCES)" RUST_SOURCES="$(RUST_SOURCES)" KERNEL_MODULE="$(KERNEL_MODULE)" DIR=$(DIR) BUILD_TYPE=$(BUILD_TYPE) EXTRAFLAGS="$(EXTRAFLAGS)" $(MAKE) -C "$(KERNEL_DIR)" M="$(BUILD_DIRECTORY)" modules
	cp "$(BUILD_DIRECTORY)/$(KERNEL_MODULE).ko" "$(KERNEL_MODULE).ko"

$(BUILD_DIRECTORY)/Makefile: module.mk
	@mkdir -p "$(BUILD_DIRECTORY)/src"
	cp "module.mk" "$(BUILD_DIRECTORY)/Makefile"

clean:
	rm -rf "$(BUILD_DIRECTORY)" || true
	rm iotd.ko || true
	rm include/iotd/rs/provided.h || true

.PHONY: all modules iotd clean insmod rmmod test