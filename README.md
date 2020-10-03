# [Tutorial][Tutorial]

[Tutorial]: https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/
[SparkPost]: https://www.sparkpost.com/
[お名前.com]: https://www.onamae.com/
[Sending Domains]: https://app.sparkpost.com/account/sending-domains
[DNSレコード設定]: https://www.onamae.com/domain/navi/dns_controll/input

# How to run

## server

```
echo -n -e "\
DATABASE_URL=postgres://username:password@localhost/your_database\n\
SPARKPOST_API_KEY='yourapikey'\n\
SENDING_EMAIL_ADDRESS='register@yourdomain.com'\n\
" > .env
```
then
```
cargo run
```

## client

* Invitation request
```
curl --request POST \
  --url http://localhost:3000/api/invitation \
  --header 'content-type: application/json' \
  --data '{"email":"name@domain.com"}'
```

![invited](images/invited.png)

* Registration request
```
curl --request POST \
  --url http://localhost:3000/api/register/964e9cc1-78af-4ca6-9d63-86f578086b4e \
  --header 'content-type: application/json' \
  --data '{"password":"password"}'
```

![registered](images/registered.png)

* Login request
```
curl -i --request POST \
  --url http://localhost:3000/api/auth \
  --header 'content-type: application/json' \
  --data '{"email": "name@domain.com","password":"password"}'
```

![logged_in](images/logged_in.png)

* Logout request
```
curl -i --request DELETE \
  --url http://localhost:3000/api/auth
```

![logged_out](images/logged_out.png)

# Log

## PREREQUISITE

ひとまず `postgresql` `diesel_cli` を無視して先へ

## LET’S BEGIN

環境を確認
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
そのあと色々やるも、どうも `argonautica` がらみで build できない  
代わりに `rust-argon2` を入れて、ひとまず build に成功

## SETUP THE BASE APP


`models.rs` を作成、 `main.rs` を編集  
`postgresql` `diesel_cli` が必要になるので用意
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

とのことですが、まだ `schema.rs` `utils.rs` `invitation_handler.rs` を作っていないので  
コンパイルは通らないはずです

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

`errors.rs` を作成

## IMPLEMENTING HANDLERS

`invitation_handler.rs` を作成

## TEST YOUR SERVER

そもそもまだ build できてないので `curl` しようがないですね

## USING SPARKPOST TO SEND REGISTRATION EMAIL

`.env` に追記、下表はそのためにやったこと

| どこで | なにを |
| - | - |
| [SparkPost][SparkPost] | アカウントを作る |
| [お名前.com][お名前.com] | 安いドメインを買う |
| SparkPost | [送信ドメイン][Sending Domains] に追加、 `DNS Settings` を確認 |
| お名前.com | `DNS Settings` を [DNSレコード設定][DNSレコード設定] に追加登録 |
| SparkPost | すこし待ってから `Verify TXT Record` |
| SparkPost | APIキー生成 |

`email_service.rs` を作成  
`invitation_handler.rs` のコメントアウト
```
// use crate::email_service::send_invitation;
// send_invitation(&invitation)
```
を解除

## GET SOME HELP

`utils.rs` を作成  
* `argonautica` の代わりに `rust-argon2` で書いてみました

`models.rs` を編集、
```
use serde::{Serialize, Deserialize};
```
を追記

`email_service.rs` の `Transmission::new_eu` を `Transmission::new` に変更  
`cargo run` するとサーバが起動するので別ターミナルから `curl` を叩きます
```
curl --request POST \
  --url http://localhost:3000/api/invitation \
  --header 'content-type: application/json' \
  --data '{"email":"name@domain.com"}'
```

登録確認メールが届きました！

![invitation_mail](images/invitation_mail.png)

## REGISTERING USER

`register_handler.rs` を作成

## PARSING URL PARAMETERS

`main.rs` を編集

## TEST YOUR IMPLEMENTATION

```
curl --request POST \
  --url http://localhost:3000/api/invitation \
  --header 'content-type: application/json' \
  --data '{"email":"name@domain.com"}'
```

![invited](images/invited.png)

```
curl --request POST \
  --url http://localhost:3000/api/register/964e9cc1-78af-4ca6-9d63-86f578086b4e \
  --header 'content-type: application/json' \
  --data '{"password":"password"}'
```

![registered](images/registered.png)

## AUTH HANDLING

`auth_handler.rs` を作成、 `main.rs` を編集

## TEST RUN AUTH

```
curl -i --request POST \
  --url http://localhost:3000/api/auth \
  --header 'content-type: application/json' \
  --data '{"email": "name@domain.com","password":"password"}'
```

![logged_in](images/logged_in.png)

```
curl -i --request DELETE \
  --url http://localhost:3000/api/auth
```

![logged_out](images/logged_out.png)

## TESTING LOGGED IN USER
