# @lnp2pBot API

Rust API to connect with [@lnp2pBot](https://github.com/lnp2pBot/bot) in real time

## Requirements:

0. You need Rust version 1.48 or higher to compile it.

## Install

Clone the repository and then create a new `.env` file based on `.env-sample` file.

```
$ git clone https://github.com/lnp2pBot/rust-api.git
$ cd rust-api
$ cp .env-sample .env
```

To connect with a mongodb server we need to set 2 variables in the `.env` file, `DATABASE_URL` and `DATABASE_NAME`.

## Compile and execute it:

To compile on Ubuntu/Pop!\_OS, please install [cargo](https://www.rust-lang.org/tools/install), then run the following commands:

```
$ cargo build --release
$ target/release/rust-api
```

Go to http://localhost:8000

With curl:

```
$ curl -X GET -H "Content-type: application/json" -d '{}' "http://localhost:8000/orders"
```
