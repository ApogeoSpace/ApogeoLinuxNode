# Example - Python

This directory contains python code to interact with the node hat.

## node.py

The script implements a simple node transmitting a constant payload at given intervals.
the following command line options are allowed:
```
  -h, --help                                show this help message and exit
  -n NODE_ID, --node-id NODE_ID             Node id to be used, decimal format
  -k KEY_FILE, --key-file KEY_FILE          Key file to be loaded
  -f {bin,hex}, --key-format {bin,hex}      Key format to be used
  -t INTERVAL, --interval INTERVAL          Interval, in secods, the node will transmit
  -p PAYLOAD, --payload PAYLOAD             hex payload to be transmitted. Will be cropped at 10 bytes if longer, zero filled if shorted
  -F {hex,str}, --payload-format {hex,str}  Payload format
  -d DEVICE, --device DEVICE                Device name
```

When run, the script will load the node key from the specified file using the format specified by `key-format`.
Once the key has been loaded, the script will configure the device identified by the relative flag with the
node id and key obtained by parsing the options and then will start transmitting, using the system's date time as
clock, every `interval` second the payload defined by the relative option.

Payload format can be specified as `str`, meaning the payload will be written as-is into the message, or `hex`, where the
payload will be decoded as an hex string before transmission.

### Example command
```
sudo python node.py -n 42 -k key.txt -f hex -t 10 -p "Hi APS!" -F str -d iotd0
```

Output
```
Loading id
Loading key
Sending payload HEX(48692041505321000000) every 10 seconds
Transmitting in 10
Transmitting in 9
Transmitting in 8
Transmitting in 7
...
```

## receive.py

This tool starts listening with the system's configured lora parameters and will display the received messages.
The following command line options are allowed:
```
-h, --help                          show this help message and exit
-d DEVICE, --device DEVICE          Device name
```

### Example command
```
sudo python receive.py -d iotd0
```

Output
```
================================ Received  30 Bytes ================================
===================== RSSI  -89 == SNR   11 == FEI 68.6476 kHz =====================
000000:  00 00 00 00 00 00 02 08  06 95 81 0a 0d 82 13 04  |  . . . . . . . .  . . . . . . . .
000010:  05 95 13 1e 94 92 15 00  96 10 8f 1b 89           |  . . . . . . . .  . . . . .

================================ Received  30 Bytes ================================
===================== RSSI  -89 == SNR   11 == FEI 68.6456 kHz =====================
000000:  00 00 00 00 00 00 02 08  06 95 8b 0a 01 10 96 89  |  . . . . . . . .  . . . . . . . .
000010:  81 16 8e 0f 13 99 81 06  19 0d 13 11 8d           |  . . . . . . . .  . . . . .

================================ Received  30 Bytes ================================
===================== RSSI  -91 == SNR   11 == FEI 68.6456 kHz =====================
000000:  00 00 00 00 00 00 02 08  06 95 95 0a 0e 15 8d 1c  |  . . . . . . . .  . . . . . . . .
000010:  8a 8f 0e 11 80 0a 91 02  07 86 83 0c 9e           |  . . . . . . . .  . . . . .
```