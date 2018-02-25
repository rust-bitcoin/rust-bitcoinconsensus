# Bitcoin's libbitcoinconsensus with Rust binding

This project offers Rust binding to a library built from Bitcoin's C++ sources using cargo.

Bitcoin's own build creates a library called libbitcoinconsenus, that allows transaction verification using Bitcoins unique script engine. Bitcoin enabled applications SHOULD use libbitcoinconsensus library to avoid accepting transactions that the Bitcoin network nodes would not.

This project simplifies Rust developer's life by creating the libbitcoinconsensus library with cargo. No need to deal with the archaic C++ toolchain directly.  This also simplifies cross-compiling the consenus library e.g. for mobile application.

## Build

This project has a submodule, you have to clone it using:

`
git clone --recurse-submodules git@github.com:tamasblummer/rust-bitcoinconsensus.git
`

then build it simple with:

`
cargo build
`

I verified the build for Linux and OSX. PRs are welcome to extend support for other platforms.


## API
The API is very basic, exposing Bitcoin's as is. This is intentional to keep this project minimal footprint and no further runtime dependencies. You will need another Rust library to serialize Bitcoin transactions and transaction outputs.

Verify a single spend (input) of a Bitcoin transaction.

`
verify (spent_output_script: &[u8], amount: u64, spending_transaction: &[u8], input_index: usize) -> Result<(), Error>
`

### Arguments
 * spend_output_script: a Bitcoin transaction output script to be spent, serialized in Bitcoin's on wire format
 * amount: The spent output amount in satoshis
 * spending_transaction: spending Bitcoin transaction, serialized in Bitcoin's on wire format
 * input_index: index of the input within spending_transaction
### Example

The (randomly choosen) Bitcoin transaction [aca326a724eda9a461c10a876534ecd5ae7b27f10f26c3862fb996f80ea2d45d](https://blockchain.info/tx/aca326a724eda9a461c10a876534ecd5ae7b27f10f26c3862fb996f80ea2d45d)
spends one input, that is the first output of [95da344585fcf2e5f7d6cbf2c3df2dcce84f9196f7a7bb901a43275cd6eb7c3f](https://blockchain.info/tx/95da344585fcf2e5f7d6cbf2c3df2dcce84f9196f7a7bb901a43275cd6eb7c3f) with a value of 630482530 satoshis

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

should return OK(())

_Note_ that spent amount can only be checked for Segwit transactions.