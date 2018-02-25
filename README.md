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

`
pub fn verify (spent_output: &[u8], amount: u64, spending_transaction: &[u8], input_index: usize) -> Result<(), Error>
`