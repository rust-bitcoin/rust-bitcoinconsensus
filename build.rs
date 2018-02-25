extern crate gcc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut base_config = gcc::Build::new();
    base_config.cpp(true)
        .include("bitcoin/src")
        .include("bitcoin/src/secp256k1/include")
        .flag("-g").flag("-O2").flag("-std=c++11").flag("-Wno-unused-parameter")
        .define("__STDC_FORMAT_MACROS", None);
    if target == "x86_64-unknown-linux-gnu" {
	base_config
        .define("HAVE_ENDIAN_H", Some("1"))
        .define("HAVE_DECL_HTOBE16", Some("1"))
        .define("HAVE_DECL_HTOLE16", Some("1"))
        .define("HAVE_DECL_BE16TOH", Some("1"))
        .define("HAVE_DECL_LE16TOH", Some("1"))
        .define("HAVE_DECL_HTOBE32", Some("1"))
        .define("HAVE_DECL_HTOLE32", Some("1"))
        .define("HAVE_DECL_BE32TOH", Some("1"))
        .define("HAVE_DECL_LE32TOH", Some("1"))
        .define("HAVE_DECL_HTOBE64", Some("1"))
        .define("HAVE_DECL_HTOLE64", Some("1"))
        .define("HAVE_DECL_BE64TOH", Some("1"))
        .define("HAVE_DECL_LE64TOH", Some("1"));
    }
    base_config
        .file("bitcoin/src/utilstrencodings.cpp")
        .file("bitcoin/src/uint256.cpp")
        .file("bitcoin/src/pubkey.cpp")
        .file("bitcoin/src/primitives/transaction.cpp")
        .file("bitcoin/src/crypto/ripemd160.cpp")
        .file("bitcoin/src/crypto/sha1.cpp")
        .file("bitcoin/src/crypto/sha256.cpp")
        .file("bitcoin/src/script/bitcoinconsensus.cpp")
        .file("bitcoin/src/script/interpreter.cpp")
        .file("bitcoin/src/script/script.cpp")
        .file("bitcoin/src/script/script_error.cpp")
        .compile("libbitcoinconsensus.a");
}
