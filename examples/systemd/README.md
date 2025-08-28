# Systemd example

This directory contains an example setup implementing a transmitting node with constant interval.
The transmission is handled by a systemd timer and service.

## Installation
```
sudo make DEVNAME=iotd0 BOOT_DELAY=1min INTERVAL=3min INSTALL_PREFIX=/usr/local install
sudo make enable
sudo make start
```

See makefile for other useful targets.

## Interface
This node implementation sends the content of the file `<INSTALL_PREFIX>/var/<DEVNAME>` repeatedly.

## Configuration
Node parameters can be configured in `<INSTALL_PREFIX>/etc/iotd/<DEVNAME>/*`. The directory structure mirrors the
format of the configuration directory described in `README.md` and is applied on boot.

