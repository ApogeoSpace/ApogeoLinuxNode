INCLUDE_FLAGS := -I $(DIR)/include
WARN_FLAGS := -Wall -Werror
EXTRA_CFLAGS += $(INCLUDE_FLAGS) $(WARN_FLAGS) $(EXTRAFLAGS)

rust-target := lib$(KERNEL_MODULE).a

obj-m                 := $(KERNEL_MODULE).o
$(KERNEL_MODULE)-objs := $(MODULE_SOURCES:.c=.o) $(rust-target)

$(MODULE_SOURCES:%=$(M)/%): $(MODULE_SOURCES:%=$(DIR)/%)
	@[ ! -e $@ ] && ln -s $(@:$(M)/%=$(DIR)/%) $@ || true

ifeq ($(BUILD_TYPE),release)
	EXTRA_LDFLAGS += --strip-all
	RELEASE = 1
endif

CARGO_MOD_DIR := $(TOPDIR)/target/$(UTS_MACHINE)-unknown-none-gnu/$(if $(RELEASE),release,debug)
CARGO_BLD_DIR := $(TOPDIR)/target/$(if $(RELEASE),release,debug)

$(obj)/$(rust-target): $(RUST_SOURCES) FORCE
	@cd "$(TOPDIR)/iotd" && RUST_TARGET_PATH='$(BASE_DIR)' STD_CLANG_ARGS='$(c_flags)' STD_KERNEL_PATH='$(CURDIR)' STD_CLANG_FILES='$(KERNEL_INCLUDE)' TOPDIR='$(TOPDIR)' cargo rustc -Zjson-target-spec $(if $(RELEASE),--release) $(if $(V),--verbose) $(CARGOFLAGS) --target="../.cargo/$(UTS_MACHINE)-unknown-none-gnu.json" -- $(RCFLAGS)
	@cp "$(CARGO_MOD_DIR)/$(rust-target)" $(obj)
	touch $(obj)/.$(rust-target).cmd
