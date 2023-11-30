[![Status](https://travis-ci.org/rust-bitcoin/rust-bitcoinconsensus.png?branch=master)](https://travis-ci.org/rust-bitcoin/rust-bitcoinconsensus)

# Bitcoin's `libbitcoinconsensus` with Rust bindings

This project builds the `libbitcoinconsensus` library from Bitcoin's C++ sources using cargo and provides Rust bindings to its API.

`libbitcoinconsensus` allows transaction verification using Bitcoin's unique script engine.
Bitcoin enabled applications SHOULD use the `libbitcoinconsensus` library to avoid accepting transactions that the Bitcoin network nodes would not accept.

This project simplifies a Rust developer's life by creating the `libbitcoinconsensus` library with cargo.
No need to deal with the archaic C++ toolchain directly.
This also simplifies cross-compiling the consensus library e.g., for a mobile application.

`libbitcoinconsensus` refers to code from another library [secp256k1](https://github.com/bitcoin-core/secp256k1).
A snapshot of that library is also included in the Bitcoin sources, therefore it could be baked into `libbitcoinconsensus`.
A typical Bitcoin enabled application will however want to access further secp256k1 functions.
The project [rust-secp256k1](https://github.com/rust-bitcoin/rust-secp256k1) offers a cargo build and Rust bindings, therefore we depend on that instead of compiling the Bitcoin embedded sources into `libbitcoinconsensus`q.
This introduces a risk, since a difference between the two secp256k1 sources could break consensus with Bitcoin.


## Version numbers

We use [slightly abuse] semantic versioning. The first `Major.Minor.Patch` number tracks the vendored Bitcoin Core code (see below), the second `Major.Minor.Patch` tracks this crate. 
For example, if we upgrade the Bitcoin Core code by a `Patch` version we also bump our `Patch` version.

One side effect of this is that `crates.io` shows our release versions in yellow as if they were pre-release versions, this is due to us using a `-` which, in semantic versioning, implies a pre-release version.


## Vendor Bitcoin Core

We use a script to vendor the Bitcoin Core code, the script takes the
Bitcoin Core version number to vendor: `./vendor-bitcoin-core.sh 0.21.1`


## MSRV

The MSRV of this crate is **1.48.0**.


## Githooks

To assist devs in catching errors _before_ running CI we provide some githooks. If you do not
already have locally configured githooks you can use the ones in this repository by running, in the
root directory of the repository:
```
git config --local core.hooksPath githooks/
```

Alternatively add symlinks in your `.git/hooks` directory to any of the githooks we provide.


## API

The API is very basic, exposing Bitcoin's API as is.
This is intentional to keep this project to a minimal footprint and add no further runtime dependencies.
You will need another Rust library to serialize Bitcoin transactions and scripts.

Verify a single spend (input) of a Bitcoin transaction:

`
verify (spent_output_script: &[u8], amount: u64, spending_transaction: &[u8], input_index: usize) -> Result<(), Error>
`


### Arguments

 * spend_output_script: a Bitcoin transaction output script to be spent
 * amount: The spent output amount in satoshis
 * spending_transaction: spending Bitcoin transaction, serialized in Bitcoin's on wire format
 * input_index: index of the input within spending_transaction


### Example

The (randomly chosen) Bitcoin transaction [aca326a724eda9a461c10a876534ecd5ae7b27f10f26c3862fb996f80ea2d45d](https://blockchain.info/tx/aca326a724eda9a461c10a876534ecd5ae7b27f10f26c3862fb996f80ea2d45d) spends one input, that is the first output of [95da344585fcf2e5f7d6cbf2c3df2dcce84f9196f7a7bb901a43275cd6eb7c3f](https://blockchain.info/tx/95da344585fcf2e5f7d6cbf2c3df2dcce84f9196f7a7bb901a43275cd6eb7c3f) with a value of 630482530 satoshis.

The spending transaction in wire format is:

`
spending = 02000000013f7cebd65c27431a90bba7f796914fe8cc2ddfc3f2cbd6f7e5f2fc854534da95000000006b483045022100de1ac3bcdfb0332207c4a91f3832bd2c2915840165f876ab47c5f8996b971c3602201c6c053d750fadde599e6f5c4e1963df0f01fc0d97815e8157e3d59fe09ca30d012103699b464d1d8bc9e47d4fb1cdaa89a1c5783d68363c4dbc4b524ed3d857148617feffffff02836d3c01000000001976a914fc25d6d5c94003bf5b0c7b640a248e2c637fcfb088ac7ada8202000000001976a914fbed3d9b11183209a57999d54d59f67c019e756c88ac6acb0700
`

The script of the first output of the spent transaction is:

`
spent = 76a9144bfbaf6afb76cc5771bc6404810d1cc041a6933988ac
`

The (pseudo code) call:

`
verify(spent, 630482530, spending, 0)
`

should return `Ok(())`

**Note** that spent amount will only be checked for Segwit transactions.
The above example is not segwit therefore verify will succeed with any amount.
