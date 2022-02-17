# ip2d

A converter for IPv4 addresses.

## Usage

```console
ip2d 0.3.0
0xflotus
A converter for IPv4 Addresses

USAGE:
    ip2d [OPTIONS] [ip]

ARGS:
    <ip>    Converts an IPv4 Address to an integer

OPTIONS:
    -h, --help                Print help information
    -r, --reverse <number>    Converts an integer to an IPv4 Address
    -V, --version             Print version information
    -x, --hex                 Converts an IPv4 Address to a hex number
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
