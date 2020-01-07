extern crate cc;

use std::env;

fn main() {
    // Check whether we can use 64-bit compilation
    let use_64bit_compilation = if env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "64" {
        let check = cc::Build::new()
            .file("depend/check_uint128_t.c")
            .cargo_metadata(false)
            .try_compile("check_uint128_t")
            .is_ok();
        if !check {
            println!("cargo:warning=Compiling in 32-bit mode on a 64-bit architecture due to lack of uint128_t support.");
        }
        check
    } else {
        false
    };
    let mut base_config = cc::Build::new();
    base_config
        .cpp(true)
        .include("depend/bitcoin/src")
        .include("depend/bitcoin/src/secp256k1/include")
        .define("__STDC_FORMAT_MACROS", None)
        // **Secp256k1**
        .include("depend/bitcoin/src/secp256k1")
        .flag_if_supported("-Wno-unused-function") // some ecmult stuff is defined but not used upstream
        .define("SECP256K1_BUILD", "1")
        // Bitcoin core defines libsecp to *not* use libgmp.
        .define("USE_NUM_NONE", "1")
        .define("USE_FIELD_INV_BUILTIN", "1")
        .define("USE_SCALAR_INV_BUILTIN", "1")
        // Technically libconsensus doesn't require the recovery feautre, but `pubkey.cpp` does.
        .define("ENABLE_MODULE_RECOVERY", "1");

    if use_64bit_compilation {
        base_config
            .define("USE_FIELD_5X52", "1")
            .define("USE_SCALAR_4X64", "1")
            .define("HAVE___INT128", "1");
    } else {
        base_config
            .define("USE_FIELD_10X26", "1")
            .define("USE_SCALAR_8X32", "1");
    }

    let tool = base_config.get_compiler();
    if tool.is_like_msvc() {
        base_config.flag("/std:c++14").flag("/wd4100");
    } else if tool.is_like_clang() || tool.is_like_gnu() {
        base_config.flag("-std=c++11").flag("-Wno-unused-parameter");
    }
    #[cfg(target_os = "windows")]
    {
        base_config.define("WIN32", Some("1"));
    }
    base_config
        .file("depend/bitcoin/src/utilstrencodings.cpp")
        .file("depend/bitcoin/src/uint256.cpp")
        .file("depend/bitcoin/src/pubkey.cpp")
        .file("depend/bitcoin/src/hash.cpp")
        .file("depend/bitcoin/src/primitives/transaction.cpp")
        .file("depend/bitcoin/src/crypto/ripemd160.cpp")
        .file("depend/bitcoin/src/crypto/sha1.cpp")
        .file("depend/bitcoin/src/crypto/sha256.cpp")
        .file("depend/bitcoin/src/crypto/sha512.cpp")
        .file("depend/bitcoin/src/crypto/hmac_sha512.cpp")
        .file("depend/bitcoin/src/script/bitcoinconsensus.cpp")
        .file("depend/bitcoin/src/script/interpreter.cpp")
        .file("depend/bitcoin/src/script/script.cpp")
        .file("depend/bitcoin/src/script/script_error.cpp")
        .file("depend/bitcoin/src/secp256k1/src/secp256k1.c")
        .compile("libbitcoinconsensus.a");
}
