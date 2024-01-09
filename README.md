# ip2d

A converter for IPv4 addresses.

## Usage

```console
A converter for IPv4 Addresses

Usage: ip2d [OPTIONS] [ip]

Arguments:
  [ip]  Converts an IPv4 Address to an integer

Options:
  -r, --reverse <number>  Converts an integer to an IPv4 Address
  -x, --hex               Converts an IPv4 Address to a hex number
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
````
