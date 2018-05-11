extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut base_config = cc::Build::new();
    base_config
        .cpp(true)
        .include("bitcoin/src")
        .include("bitcoin/src/secp256k1/include")
        .opt_level(2)
        .debug(true)
        .define("__STDC_FORMAT_MACROS", None);

    let tool = base_config.get_compiler();
    if tool.is_like_msvc() {
        base_config.flag("/std:c++14").flag("/wd4100");
    } else if tool.is_like_clang() || tool.is_like_gnu() {
        base_config.flag("-std=c++11").flag("-Wno-unused-parameter");
    }

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
        .file("bitcoin/src/hash.cpp")
        .file("bitcoin/src/primitives/transaction.cpp")
        .file("bitcoin/src/crypto/ripemd160.cpp")
        .file("bitcoin/src/crypto/sha1.cpp")
        .file("bitcoin/src/crypto/sha256.cpp")
        .file("bitcoin/src/crypto/sha512.cpp")
        .file("bitcoin/src/crypto/hmac_sha512.cpp")
        .file("bitcoin/src/script/bitcoinconsensus.cpp")
        .file("bitcoin/src/script/interpreter.cpp")
        .file("bitcoin/src/script/script.cpp")
        .file("bitcoin/src/script/script_error.cpp")
        .compile("libbitcoinconsensus.a");
}
