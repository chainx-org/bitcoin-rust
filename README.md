# The ChainX's bitcoin(BTC/BCH) relay inherited from Parity bitcoin client.
## Installing from source

Installing `chainx_btc` from source requires `rustc` and `cargo`.

Minimal supported version is `rustc 1.23.0 (766bd11c8 2018-01-01)`

#### Install rustc and cargo

Both `rustc` and `cargo` are a part of rust tool-chain.

An easy way to install the stable binaries for Linux and Mac is to run this in your shell:

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

Windows binaries can be downloaded from [rust-lang website](https://www.rust-lang.org/en-US/downloads.html).

#### Install C and C++ compilers

You will need the cc and gcc compilers to build some of the dependencies.

```
sudo apt-get update
sudo apt-get install build-essential
```

#### Clone and build chainx_btc

Now let's clone `chainx_btc` and enter it's directory:

```
git clone https://github.com/chainx-org/bitcoin-relay
cd bitcoin-relay
```

`chainx_btc` can be build in two modes. `--debug` and `--release`. Debug is the default.

```
# builds chainx_btc in debug mode
cargo build
```

```
# builds chainx_btc in release mode
cargo build --release
```

`chainx_btc` is now available at either `./target/debug/chainx_btc` or `./target/release/chainx_btc`.


### Base operator
To start syncing the testnet:
```
./target/release/chainx_btc --btc --testnet
```

getbestblockhash
```
{"jsonrpc": "2.0", "method": "getbestblockhash", "params": [], "id":1 }
{
    "jsonrpc": "2.0",
    "result": "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943",
    "id": 1
}
```
getblock
```
{"jsonrpc": "2.0", "method": "getblock", "params": ["000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943", true], "id":1 }
```
createrawtransaction
```
{"jsonrpc": "2.0", "method": "createrawtransaction", "params": [[{"txid":"ddb1bfb7ceb0f76e86e21b4784e9390cb9fb506c19f74bfa2275f79631d72a66","vout":0}],{"mxjL3DAjJdyWoJf2MdQamCjZx6PuAyL16k":0.01}], "id":1 }
```
signrawtransaction
```
{"jsonrpc": "2.0", "method": "signrawtransaction", "params": ["0100000001662ad73196f77522fa4bf7196c50fbb90c39e984471be2866ef7b0ceb7bfb1dd0000000000ffffffff0140420f00000000001976a914bcd147e9a3845755a62c1483fabedf23490c115c88ac00000000", "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943"], "id":1 }
```
sendrawtransaction
```
{"jsonrpc": "2.0", "method": "sendrawtransaction", "params": ["0100000001662ad73196f77522fa4bf7196c50fbb90c39e984471be2866ef7b0ceb7bfb1dd000000006b48304502210092c5ac9178de7c7e959b114df024d6fb3d4b09b63067b8af554f42cff9c28ea202204a6e6ea9bd1b8d7c41dfcd7dbaa2c35dce7bf2d500591739b2577de6fae1e5a94121038d3c8f507cc730ddd3c0dd3aafafbd2d53f6d3a17d5a78f2e6c4ee8ae50bc8f8ffffffff0140420f00000000001976a914bcd147e9a3845755a62c1483fabedf23490c115c88ac00000000"], "id":1 }
```

coinbase private secrect: 000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943
