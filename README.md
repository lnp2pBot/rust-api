# @lnp2pBot API

Rust API to connect with [@lnp2pBot](https://github.com/lnp2pBot/bot) in real time

## Install

Clone the repository and then create a new `.env` file based on `.env-sample` file.

```
$ git clone https://github.com/lnp2pBot/rust-api.git
$ cd rust-api
$ cp .env-sample .env
```

To connect with a mongodb server we need to set 2 variables in the `.env` file, `DATABASE_URL` and `DATABASE_NAME`.

## Requirements:

0. You need Rust version 1.48 or higher to compile it.

## Compile and execute it:

```
$ cargo build
$ target/debug/rust-api
```

Go to http://localhost:8000/orders
