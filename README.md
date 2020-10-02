# [Tutorial][tutorial]

# Log

## PREREQUISITE

ひとまず `postgresql` `diesel_cli` を無視して先へ

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
<!-- $ sudo apt install libssl-dev pkg-config -->
```
この後色々やるがどうも `argonautica` がらみで build できない  
`rust-argon2` に代えて、ひとまず build に成功

## SETUP THE BASE APP

ここで `postgresql` `diesel_cli` が必要になるので用意

```
$ sudo apt-get install postgresql-12
<!-- $ sudo apt install libpq-dev -->
$ cargo install diesel_cli --no-default-features --features postgres
```

データベース接続情報を用意

```
$ sudo passwd postgres
$ sudo -u postgres createdb simple_auth
$ echo DATABASE_URL=postgres://postgres:password@localhost/simple_auth > .env
```
> At this stage your server should compile

とあるが、まだコンパイルは通らないはずです

```
error[E0583]: file not found for module `schema`
  --> src/main.rs:10:1
   |
10 | mod schema;
   | ^^^^^^^^^^^
   |
   = help: to create the module `schema`, create file "src/schema.rs"

error[E0433]: failed to resolve: use of undeclared type or module `utils`
  --> src/main.rs:36:43
   |
36 |                 CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
   |                                           ^^^^^ use of undeclared type or module `utils`

error[E0433]: failed to resolve: use of undeclared type or module `invitation_handler`
  --> src/main.rs:50:51
   |
50 | ...                   .route(web::post().to(invitation_handler::post_invitation)),
   |                                             ^^^^^^^^^^^^^^^^^^ use of undeclared type or module `invitation_handler`
```

## SETTING UP DIESEL AND CREATING OUR USER MODEL


[tutorial]: https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/
