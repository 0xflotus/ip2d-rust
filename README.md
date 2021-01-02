# IP2D

A converter for IPv4 addresses.

## Usage

```console
A converter for IPv4 Addresses

USAGE:
    ip2d [FLAGS] [OPTIONS] [ip]

ARGS:
    <ip>    Converts an IPv4 Address to a number

FLAGS:
    -h, --help       Prints help information
    -x, --hex        Converts an IPv4 Address to a hex number
    -V, --version    Prints version information

OPTIONS:
    -r, --reverse <number>    Converts a number to an IPv4 Address
```

## Examples

`ip2d 127.0.0.1` -> `2130706433`

`ip2d -r 230451` -> `0.3.132.51`
