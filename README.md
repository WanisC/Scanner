# Network Scanner

## Overview

The aim of this project is to develop a network scanner that can be used to find out which devices are connected to the same network as the launcher.

### Notice of non-professional use

This project has been developed purely for educational purposes and its functionalities may not be suitable for professional environments or critical situations. Consequently, I decline all responsibility for damage, loss of data or any other prejudice resulting from inappropriate or unauthorized use of this project in professional contexts.

### Part 1: IPV4

The IPV4 address scanning feature naturally allows you to find (local) addresses within the network you're on by giving the first 2 pairs of bits.

### Part 2: 6to4

Now the 6to4 address scan functionality (IPV4 to IPV6 transition method) has the same utility as previously described, but we've now added IPV6 address visualization.

![Encoding an IPv4 address in the 6to4 prefix.](https://upload.wikimedia.org/wikipedia/commons/thumb/9/96/6to4_convert_address.svg/330px-6to4_convert_address.svg.png)

### Part 3: Port

This feature scans the IPV4 addresses connected to the port given as a parameter.

## How to Use

You can either use the target in the Makefile or enter the entire command:

```bash
make ipv4 ARGS="192 168"
cargo run --release scan-ipv4 192 168

make ipv6 ARGS="192 168"
cargo run --release scan-ipv6 192 168

make port ARGS="192 168 -p 80"
cargo run --release scan-port 192 168 -p 80
```

## Version/Language

- Current version: v2023-2024
- Language: Rust
