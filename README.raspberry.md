# APS Node Linux - Raspberry

The build process has been tested for the following hardware:
 - Raspberry Pi 4
 - Raspberry Pi Zero (W)

# Build & Install

## Requirements

Install required development packages with the following command

```
sudo apt update
sudo apt install git clang make gcc build-essential linux-headers
```

Install rust toolchain and cbindgen
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env" 
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src
cargo install cbindgen
```

# Build & Test

Clone the repository and change directory
```
git clone --depth 1 --branch master https://git.intra.apogeo.space/ApogeoSpace/aps-node-linux.git iotd
cd iotd
```
then build the module
```
make iotd
```

*NOTE*: If using 32 bit cpu (Raspberry PI Zero / Raspberry Pi 1), use `iotd-32` target instead.

## Device tree overlay

For Raspberry PI 4:

Modify the file "device_tree/iot_device.dts" according to the comments in it.
Build the overlay with
```
make dt_overlay
```

# Testing
To test the module execute the following:
First load the device tree overlay at runtime, substituting the device tree name you previously edited
```
sudo dtoverlay device_tree/iot_device.dtbo
```

Load the module
```
sudo insmod module/build/iotd.ko
```

If problems arise, enable kernel debug logs with
```
echo 7 | sudo tee /proc/sys/kernel/printk
```

Test transmission and reception with
```
echo "hi-aps" | sudo tee /dev/iotd0
sudo head -c 37 /dev/iotd0
```

Unload the module
```
sudo rmmod iotd
```

# Install
Install the compiled module and device tree overlay in the system
```
sudo mkdir /lib/modules/$(uname -r)/kernel/drivers/iot/
xz -v module/build/iotd.ko
sudo cp module/build/iotd.ko.xz /lib/modules/$(uname -r)/kernel/drivers/iot/iotd.ko.xz
sudo depmod -a
echo iotd | sudo tee -a /etc/modules
```

Test if modprobe finds the module
```
sudo modinfo iotd
```

Install device tree overlay and add it to the boot config
```
sudo cp device_tree/<dts file name>.dtbo /boot/overlays/iot-device.dtbo
echo "dtoverlay=iot-device" | sudo tee -a /boot/firmware/config.txt
```

