# Build a crypto wallet using Rust

This repository shows an example of how to implement a simple crypto currency wallet using Rust. This example is for educational purposes.

On my blog I explain the whole process of building this project: [Build a crypto wallet using Rust: steps how to](https://tms-dev-blog.com/build-a-crypto-wallet-using-rust/).

The wallet can generate a secret key, public key and public address. Furthermore it shows the balance for the wallet's address. An example of sending Eth from the wallet to a different address is also shown.

## Set up and run

In order to run the project a `.env` file is required containing an endpoint for connecting to an Ethereum network using WebSockets. For example:

```
INFURA_RINKEBY_WS=wss://rinkeby.infura.io/ws/v3/08xxxxxxxxx
```

The `INFURA_RINKEBY` value is an endpoint address from [infura.io](https://infura.io), however it can be any valid address to an Ethereum network WebSocket endpoint.

Infura.io is an easy way to gain access to an Ethereum node endpoint.