DTB_SRC := /boot/firmware/bcm2711-rpi-4-b.dtb
DTB_FNAME := $(notdir $(DTB_SRC))

OVERLAYS := iot_device.dts
OVERLAYS += iot_device_multi.dts

%.dtbo: %.dts
	dtc -I dts -O dtb $^ -o $@

decompile:
	dtc -I dtb -O dts $(DTB_SRC) -o decomp/$(DTB_FNAME:.dtb=.dts)

all: $(OVERLAYS:.dts=.dtbo)

clean:
	rm $(OVERLAYS:.dts=.dtbo) || true

.PHONY: decompile clean