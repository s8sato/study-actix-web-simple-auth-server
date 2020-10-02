# [Tutorial][tutorial]

[tutorial]: https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/

# Log

## PREREQUISITE

ひとまず `postgresql` `diesel_cli` を無視して先へ

## LET’S BEGIN

環境を確認します
```
$ uname -a
Linux DESKTOP-HQJERCJ 4.19.104-microsoft-standard #1 SMP Wed Feb 19 06:37:35 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux
$ lsb_release -d
Description:    Ubuntu 20.04.1 LTS
$ rustc --version && cargo --version
rustc 1.46.0 (04488afe3 2020-08-24)
cargo 1.46.0 (149022b1d 2020-07-17)
```

やらなくてもいいかもしれないやつ
```
<!-- $ echo "1.46.0" > rust-toolchain -->
<!-- $ sudo apt install libssl-dev pkg-config -->
```

`Cargo.toml` の `[dependencies]` に追記  
そのあと色々やるも、どうも `argonautica` がらみで build できませんでした  
`rust-argon2` に代えて、ひとまず build に成功です

## SETUP THE BASE APP


`models.rs` を作成、 `main.rs` を編集  
`postgresql` `diesel_cli` が必要になるので用意します
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

とのことですが `schema.rs` `utils.rs` `invitation_handler.rs` といったモジュールまだないのでコンパイル通らないはずです

## SETTING UP DIESEL AND CREATING OUR USER MODEL

```
$ diesel setup
$ diesel migration generate users
$ diesel migration generate invitations
```

`up.sql` `down.sql` たちを編集

```
$ diesel migration run
```

`models.rs` を編集

> Check your implementation is free from errors/warnings

とありますが当然まだエラーだらけです。ガッハッハ！

## OUR OWN ERROR RESPONSE TYPE

`errors.rs` を作成、 `main.rs` に `mod errors;` を追記

## IMPLEMENTING HANDLERS

`invitation_handler.rs` を作成、 `main.rs` に `mod invitation_handler;` を追記

## TEST YOUR SERVER

そもそもまだ build できてないので `curl` しようがないですね

## USING SPARKPOST TO SEND REGISTRATION EMAIL

`.env` に追記  
`email_service.rs` を作成、 `main.rs` に `mod email_service;` を追記  
`invitation_handler.rs` のコメントアウト
```
// use crate::email_service::send_invitation;
// send_invitation(&invitation)
```
を解除

## GET SOME HELP
