# [Tutorial][tutorial]

# Log

## PREREQUISITE

ひとまず `diesel` `postgresql` を無視して先へ

## LET’S BEGIN

```
$ uname -a
Linux DESKTOP-HQJERCJ 4.19.104-microsoft-standard #1 SMP Wed Feb 19 06:37:35 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux
$ lsb_release -d
Description:    Ubuntu 20.04.1 LTS
$ rustc --version && cargo --version
rustc 1.46.0 (04488afe3 2020-08-24)
cargo 1.46.0 (149022b1d 2020-07-17)
$ echo "1.46.0" > rust-toolchain
$ sudo apt install libssl-dev pkg-config
```
この後色々やるがどうも `argonautica` がらみで build できない  
`rust-argon2` に代えて、ひとまず build に成功

## SETUP THE BASE APP



[tutorial]: https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/
