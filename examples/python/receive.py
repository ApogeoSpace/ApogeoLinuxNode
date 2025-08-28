import argparse
import os
import struct

parser = argparse.ArgumentParser(
            prog='node.py',
            description='IoT node transmit example',
            epilog='made with <3 by Apogeo Space'
        )

parser.add_argument('-d', '--device', help="Device name", type=str, default="iotd0")

def hexdump_line(data):
    hx = [("0" + hex(x)[2:])[-2:] for x in data]
    asc = [chr(x) if 32 <= x <= 126 else '.' for x in data]
    
    if len(hx) % 16 != 0:
        tgt_len = (len(hx)//16 + 1) * 16
        hx += ["  "] * (tgt_len - len(hx))
        asc += [" "] * (tgt_len - len(hx))
    
    return " ".join(hx[:8]), " ".join(hx[8:]), " ".join(asc[:8]), " ".join(asc[8:])

        
if __name__ == '__main__':
    args = parser.parse_args()
        
    if os.geteuid() != 0:
        print("Node operations must be executed as root.")
        exit(2)
        
    try:
        while True:
            with open(f"/dev/{args.device}", "rb") as dev:
                data = dev.read(256+7)
                if(len(data) < 7):
                    print("=================================== Recv Failed ===================================")
                    continue
                
                l = len(data) - 7
                rssi, snr, fei = struct.unpack("=hbf", data[:7])
                                
                print("================================ Received {:3d} Bytes ================================".format(l))
                print("===================== RSSI {:4d} == SNR {:4d} == FEI {:4.4f} kHz =====================".format(rssi, snr, fei / 1000))
                
                for i in range(0, len(data) - 7, 16):
                    print("{:06x}:  {}  {}  |  {}  {}".format(i, *hexdump_line(data[i+7: min(len(data) - 1, i+7+16)])))
                
                print()
                
    except KeyboardInterrupt as _e:
        exit(0)
        
