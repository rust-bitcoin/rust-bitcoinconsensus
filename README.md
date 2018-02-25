# Bitcoin's libbitcoinconsensus with Rust binding

This project offers Rust binding to a library built from Bitcoin's C++ sources using cargo.

Bitcoin's own build creates a library called libbitcoinconsenus, that allows transaction verification using Bitcoins unique script engine. Applications that use Bitcoin SHOULD use that library to avoid accepting transactions that Bitcoin would not.

This project simplifies Rust developer's life by creating the libconsensus library with cargo. No need to deal with C++ archaic toolchain directly.  This also simplifies cross-compiling the consenus library e.g. for mobile application.

## API
The API is very basic, exposing Bitcoin's as is. This is intentional to keep this project minimal footprint and no further runtime dependencies. You will need another Rust library to serialize Bitcoin transactions and transaction outputs.

`
pub fn verify_spend (spent_output: &[u8], amount: u64, spending_transaction: &[u8], input_index: usize, flag: u32) -> Result<(), Error>
`