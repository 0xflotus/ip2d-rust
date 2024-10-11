# ip2d

A converter for IP addresses.

## Installation

```bash
cargo install ip2d
```

## Usage

```console
A converter for IP Addresses

Usage: ip2d [OPTIONS] [ip]

Arguments:
  [ip]  Converts an IP Address to an integer

Options:
  -r, --reverse <number>  Converts an integer to an IP Address
  -x, --hex               Converts an IP Address to a hex number
  -6, --v6                Force print an IPv6
  -h, --help              Print help
  -V, --version           Print version
```

## Examples

```bash
> ip2d 127.0.0.1
2130706433

> ip2d -r 230451
0.3.132.51

> ip2d -x 127.0.2.2
0x7f000202

> ip2d a:e8:156b::34
51941356147591854558638430402641972

> ip2d -r 90392037 -6
::563:45e5

> ip2d -x ::23 -6
0x00000000000000000000000000000023
````
