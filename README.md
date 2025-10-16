# APS Node Linux

> MSRV: 1.92.0-nightly
This repo contains source code for the aps node Linux kernel module.

## Compatibility
This module is compatible with kernel version 6.12+

This module has been written to be compatible with device tree enabled systems.

# Build & Installation

## Requirements

Install the required development packages:
 - clang
 - make
 - gcc
 - build-essential
 - linux-headers

Refer to rust installation guide (here)[https://rustup.rs/], then install the nightly toolchain and additional packages

```
    . "$HOME/.cargo/env" 
    rustup toolchain install nightly
    rustup default nigthly
    rustup component add rust-src
    cargo install cbindgen
```

## Build
If your architecture is different from `aarch64`, change `.cargo/config.toml` accordingly
```
    make iotd
```

### Device tree
A device tree overlay is needed. Use `device_tree/iot_device.pi4.dts` as model to create your overlay
then,
```
    make dt_overlay
```
will build all the files in the directory.

## Test
```
    make test
```
Test coverage
 - [x] aps-crypto lib
 - [ ] sx127x interfacing lib
 - [ ] linux module
    - [ ] Unit test
    - [ ] execution tests

## Install
To be implemented.

Compiled module will be available in ./module/iotd.ko

apply device tree overlay first, `sudo dtoverlay <your overlay>.dtbo`
load the module with `sudo insmod module/build/iotd.ko <parmeters>`

## Usage

### Configuration
Once the module had been loaded, configure it by writing in files inside `/sys/kernel/iotd<x>/` or
apply a default configuration by modifying the device tree data.

#### Device tree parameters

- sf: default=7, Configures the spreading factor of the lora communication
- bw: default=5, Configures the bandwidth of the lora communication. Values may be: 
    - 0 => 7.8kHz
    - 1 => 10.4kHz
    - 2 => 15.6kHz
    - 3 => 20.8kHz
    - 4 => 31.25kHz
    - 5 => 41.7kHz
    - 6 => 62.5kHz
    - 7 => 125kHz
    - 8 => 250kHz
    - 9 => 500kHz
- cr: default=2, coding rate. 
    - 1 => 4/5
    - 2 => 4/6
    - 3 => 4/7
    - 4 => 4/8
- header: default=1, if non zero, explicit header mode is used
- crc: default=1, if non zero, payload crc will be checked and transmitted
- ldro: default=0, if non zero, ldro will be enabled
- preamble: default=8, sets the transmission and reception preamble length
- xtal: default=32000000, sets the module's xtal frequency
- tcxo: default=0, if non zero, enables tcxo compatibility on the device
- freq: default=169000000, sets the device carrier frequency
- pwr: default=100, sets the output power in dBm
- cad: default=0, if non zero perform channel activity detection before transmission
- pa_boost: default=0, if non zero, output pin will be pa_boost, rfo otherwise.
- nation_id: default=1, select the power limit nation.
    - 0 => Italy
    - 1 => Test only, minimum power
- node_id: default=0, node id

#### sysfs parameters

- spreading_factor: default=7, Configures the spreading factor of the lora communication
- bandwidth: default=5, Configures the bandwidth of the lora communication. Values may be: 
    - 0 => 7.8kHz
    - 1 => 10.4kHz
    - 2 => 15.6kHz
    - 3 => 20.8kHz
    - 4 => 31.25kHz
    - 5 => 41.7kHz
    - 6 => 62.5kHz
    - 7 => 125kHz
    - 8 => 250kHz
    - 9 => 500kHz
- coding_rate: default=2, coding rate. 
    - 1 => 4/5
    - 2 => 4/6
    - 3 => 4/7
    - 4 => 4/8
- use_header: default=1, if non zero, explicit header mode is used
- use_payload_crc: default=1, if non zero, payload crc will be checked and transmitted
- ldro: default=0, if non zero, ldro will be enabled
- preamble_length: default=8, sets the transmission and reception preamble length
- carrier_frequency: default=169000000, sets the device carrier frequency
- output_power: default=100, sets the output power in dBm
- use_cad: default=0, if non zero perform channel activity detection before transmission
- nation_id: default=1, select the power limit nation.
    - 0 => Italy
    - 1 => Test only, minimum power
- node_id: default=0, node id
- node_key: default=0, node key

#### Kernel module parameters
Loading the kernel module with the following paramters is possible

- param_no_irq: default=0, if enabled (1) polling mode will be used and read/write operation may timeout returning size 0.
- param_aps_proto: default=1, if enabled, transmission will be encrypted as per aps protocol, otherwise raw transmission will be used without payload cropping

### Use
Send a message with `echo <message> | sudo tee /dev/iotd<x>` or by writing to a file.

If aps_mode is active, the message is cropped at 10 bytes if needed and sent encrypted with the confiured node_id and key

Reception does not involve cypto protocols and can be tested with `sudo head -c 37 /dev/iotd<x>`.

Receiving is implemented as a file read where the action is blocking until a packet is receive. Once received, if the buffer length is
greater than 7, a reception header is prepended to the payload with the following definition

```
+------------+----------+------------------+
| RSSI (i16) | snr (i8) | freq_err (float) |
+------------+----------+------------------+
```

aftrer which the payload is appended. Note, the payload is trucated if necessary.

## Clean
```
    make clean
```

# Documentation

- [ ] aps-crypto
- [ ] sx127x
- [ ] Linux Module

# TODO

<<<<<<< HEAD
- [ ] Support for 5W module
=======
- [ ] Support for 5W module
>>>>>>> master
