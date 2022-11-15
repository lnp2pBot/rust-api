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
# All pending orders
$ curl -X GET "http://localhost:8000/orders" | jq
# you can send optional arguments
$ curl -X GET "http://localhost:8000/orders?_id=<_id>&direction=<direction>&currency=<currency>&community_id=<community_id>" | jq
# example to get all pending orders with fiat = ARS
$ curl -X GET "http://localhost:8000/orders?currency=ARS" | jq
# All communities
$ curl -X GET "http://localhost:8000/communities" | jq
# you can send optional arguments
$ curl -X GET "http://localhost:8000/communities?_id=<_id>&currency=<currency>" | jq
# example to get all communities that works with fiat = EUR
$ curl -X GET "http://localhost:8000/communities?currency=EUR" | jq
# Order stats
$ curl -X GET "http://localhost:8000/orders?direction=<direction>&community_id=<community_id>&date_from=<date_from>&date_to=<date_to>&status=<status>" | jq
# Example to get stats of all buy success orders from 2021-10-01 to 2021-10-31
$ curl -X GET "http://localhost:8000/orders?direction=buy&date_from=2021-10-01&date_to=2021-10-31&status=SUCCESS" | jq

```
