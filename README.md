# mkpass

Command line tool to generate passwords, built with [Rust](https://www.rust-lang.org/).

## Installation

Using [cargo](https://doc.rust-lang.org/cargo/):

```bash
cargo install mkpass
```

## Help

```bash
$ mkpass --help
Command line tool to generate passwords

Usage: mkpass [OPTIONS]

Options:
  -u                     Uppercase
  -l                     Lowercase
  -n                     Number
  -s                     Special character
  -e                     Extended special character
      --length <LENGTH>  Password length [default: 32]
      --count <COUNT>    password count [default: 1]
  -h, --help             Print help information
  -V, --version          Print version information
```

## Usage

Simply generate a password of length 32:

```bash
$ mkpass
3oUMCZGk97stX6NfQz70iaVJSBYpavmN
```

Specifies the length of the password:

```
$ mkpass --length=64
QxuO89TALuho3Pk1wuiLsfKPzTqvT63F9LG0lusio18uOQD4vym51Ko9fxDDA7Sb
```

Specify the number of passwords:

```bash
$ mkpass --count=10
XDeXtkkLALjpz9MmcVYPOa28V0Ye2XKb
9AWEQjeeonrgRtQc6d4vRVjRWkAsxZMh
9X1I0ee41xtXzmLciCnA1bHmA2khL1vT
dc64sy92rY9jnOApYey1mkluMsc2xAjU
aGNuxnNi61hige0xgoEscDmO15fzW7Rb
pwiG8eXHRsH8eklBq9b5pPgp0CPqQj0F
fDDpOrLYbJHIbI1m4Bpam7jZbnYmMcRJ
GiWpCtH5Ppx1wpkS8v6nZieHZKuoPlVO
sIPg1n7Y8KYwOyVHHBO7LQrxQcMeo2ja
mpzenLxDMtOg5D5LlDdrOcs4ycYMPxss
```

Specify the charset of passwords:

```bash
$ mkpass -luns
1PlaUp@cl&U9SLL*Oo3nsFF4&*e8uCor
```
