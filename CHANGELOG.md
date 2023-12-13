## 0.100.0+0.20.2

- Change the crate version format [#75](https://github.com/rust-bitcoin/rust-bitcoinconsensus/pull/75)

## v0.20.2-0.6.1

- Add derives on error type [#72](https://github.com/rust-bitcoin/rust-bitcoinconsensus/pull/72)

## v0.20.2-0.6.0

* Bump MSRV to rust 1.48.0 [#64](https://github.com/rust-bitcoin/rust-bitcoinconsensus/pull/64)
* De-clutter the public API by moving creating an `ffi` submodule

## v0.20.2-0.5.0

* Upgrade Bitcoin Core subtree to v0.20.2

## v0.19.2-0.4.1

* switch to using a git subtree for the bitcoin core code
* vendor bitcoin core v0.19.2

## v0.19.0-0.4.0

The major change in this version is the Minimum Supported Rust Version (MSRV) bump, we now support
an MSRV of 1.41.1, along with this change we moved to using a new version number format:
<bitcoin-core-version>-<lib-version>. This is valid semantic versioning and implies we are pre-1.0
still.

You can now use `Error` in a more ergonomic manner because we implemented `std::error::Error`. We
found and fixed an off by one error in the activation height values we were matching against. We
moved away from using AppVeyor for continuous integration and now use GitHub actions.

You can check out the following pull requests for more information:

- Bump MSRV and enable edition 2018 [#34](https://github.com/rust-bitcoin/rust-bitcoinconsensus/pull/34)
- Changed format of version number to <bitcoin-core-version>-<this-lib-version>
- Fix activation heights [#30](https://github.com/rust-bitcoin/rust-bitcoinconsensus/pull/30)
- Moved to GitHub actions instead of AppVeyor for CI [#38](https://github.com/rust-bitcoin/rust-bitcoinconsensus/pull/38)
- Implement `std::error::Error` for `Error` [#45](https://github.com/rust-bitcoin/rust-bitcoinconsensus/pull/45)
- Documentation improvements [#35](https://github.com/rust-bitcoin/rust-bitcoinconsensus/pull/35/commits)

Enjoy!

## v0.19.0-2

- Added a bunch of stuff to the exclude list to make the crate a lot smaller.
