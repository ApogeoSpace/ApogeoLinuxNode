import argparse
import os
import binascii
import time

parser = argparse.ArgumentParser(
            prog='node.py',
            description='IoT node transmit example',
            epilog='made with <3 by Apogeo Space'
        )

parser.add_argument('-n', '--node-id', help="Node id to be used, decimal format", type=int, default=100)
parser.add_argument('-k', '--key-file', help="Key file to be loaded", type=str, default="./key.txt")
parser.add_argument('-f', '--key-format', help="Key format to be used", type=str, choices=['bin', 'hex'], default="hex")
parser.add_argument('-t', '--interval', help="Interval, in secods, the node will transmit", type=int, default=10)
parser.add_argument('-p', '--payload', help="hex payload to be transmitted. Will be cropped at 10 bytes if longer, zero filled if shorted", type=str, default="Hello APS!")
parser.add_argument('-F', '--payload-format', help="Payload format", type=str, choices=['hex', 'str'], default="str")
parser.add_argument('-d', '--device', help="Device name", type=str, default="iotd0")


def load_key(file, fmt):
    flags = "r" if fmt == "hex" else "rb"
    read_size = 64 if fmt == "hex" else 32
        
    with open(file, flags) as key_file:
        key = key_file.read(read_size)
        if len(key) < read_size:
            print("Failed to read key. Key data is shorter than expected.")
            exit(1)
        
        if fmt == "hex":
            return binascii.unhexlify(key)
        return key

def load_payload(data, fmt):
    data = data.encode()
    if fmt == 'hex':
        data = binascii.unhexlify(data)
    
    # Not strictly necessary but done for cosmetic purposes
    if len(data) > 10:
        data = data[:10]
    elif len(data) < 10:
        data = data + b'\x00' * (10 - len(data))
        
    return data
        
if __name__ == '__main__':
    args = parser.parse_args()
    
    if os.geteuid() != 0:
        print("Node operations must be executed as root.")
        exit(2)
    
    key = load_key(args.key_file, args.key_format)
    pld = load_payload(args.payload, args.payload_format)
    
    if args.interval < 5:
        print("It is not good etiquette to shout on the channel!")
        exit(3)
    
    print("Loading id")
    with open(f"/sys/kernel/{args.device}/node_id", "w") as idfile:
        idfile.write(str(args.node_id))
        
    print("Loading key")
    with open(f"/sys/kernel/{args.device}/node_key", "wb") as keyfile:
        keyfile.write(key)
    
    print(f"Sending payload HEX({binascii.hexlify(pld).decode()}) every {args.interval} seconds")
    try:
        while True:
            with open(f"/dev/{args.device}", "wb") as dev:
                dev.write(pld)
                
            for i in range(args.interval):
                print(f"Transmitting in {args.interval - i}        ", end='\r')
                time.sleep(1)
    except KeyboardInterrupt as _e:
        exit(0)
        
