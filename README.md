# Ip checker

## Have u ever wonder how your device determen if it should send the packet to default Getaway or this is LAN packet.

This idea is simple and as follow.
- First take your IP and convert it into bits.
- Second take the destination IP and do the same.
- Third take the subnet mask for your device and do the same too.
- fourth, XOR your IP and the Dest IP.
- Fifth, AND the results from the XOR operation and use AND operator with the subnet mask.
- Sixth, See the magic. If the resulted AND are all zero meaning the IP is within your LAN. Other wise it mean that it out side of your LAN.


## Why did i do it.
Simple idea and i want to learn handling IPs in Rust as well as Bit wise operations.


## How to Run.

```bash

    git clone https://github.com/Cythonic1/ip_checker..git
    cd ip_checker && cargo run -- -h

```


## Command line arguments.

```bash
    -p -> your IP.
    -s -> subnet mask.
    -o -> other ip

```


