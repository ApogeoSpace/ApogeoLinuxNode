DIR := $(PWD)
TOPDIR := $(PWD)

iotd:
	DIR=$(DIR)/module TOPDIR=$(TOPDIR) make -C $(TOPDIR)/module -f Makefile iotd

iotd-32:
	DIR=$(DIR)/module TOPDIR=$(TOPDIR) EXTRAFLAGS="-Wno-pointer-to-int-cast -Wno-format" make -C $(TOPDIR)/module -f Makefile iotd


dt_overlay:
	DIR=$(DIR)/device_tree TOPDIR=$(TOPDIR) make -C $(TOPDIR)/device_tree -f Makefile all

clean:
	DIR=$(DIR)/module TOPDIR=$(TOPDIR) make -C $(TOPDIR)/module -f Makefile clean
	DIR=$(DIR)/device_tree TOPDIR=$(TOPDIR) make -C $(TOPDIR)/device_tree -f Makefile clean
	cargo clean



DBG_LEVEL := 7
DT_OVERLAY := $(DIR)/device_tree/iot_device.dtbo
insmod: iotd dt_overlay
	echo $(DBG_LEVEL) | sudo tee /proc/sys/kernel/printk > /dev/null
	sudo dtoverlay $(DT_OVERLAY)
	sudo insmod module/build/iotd.ko

rmmod:
	sudo rmmod iotd
	sudo dtoverlay -R

test:
	cd lib/aps-crypto; cargo test_lib

.PHONY: iotd dt_overlay clean insmod rmmod test
