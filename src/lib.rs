//
// Copyright 2018 Tamas Blummer
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.
//
extern crate libc;

use libc::{c_int,c_uchar, c_uint, uint64_t};

#[allow(non_camel_case_types)]
#[derive(Debug,PartialEq)]
#[repr(C)]
pub enum Error {
    ERR_SCRIPT = 0,
    #[allow(dead_code)]
    ERR_TX_INDEX,
    #[allow(dead_code)]
    ERR_TX_SIZE_MISMATCH,
    #[allow(dead_code)]
    ERR_TX_DESERIALIZE,
    #[allow(dead_code)]
    ERR_AMOUNT_REQUIRED
}

#[allow(dead_code)]
pub const VERIFY_NONE : c_uint = 0;
// evaluate P2SH (BIP16) subscripts
pub const VERIFY_P2SH : c_uint = (1 << 0);
// enforce strict DER (BIP66) compliance
pub const VERIFY_DERSIG : c_uint = (1 << 2);
// enforce NULLDUMMY (BIP147)
pub const VERIFY_NULLDUMMY : c_uint = (1 << 4);
// enable CHECKLOCKTIMEVERIFY (BIP65)
pub const VERIFY_CHECKLOCKTIMEVERIFY : c_uint = (1 << 9);
// enable CHECKSEQUENCEVERIFY (BIP112)
pub const VERIFY_CHECKSEQUENCEVERIFY : c_uint = (1 << 10);
// enable WITNESS (BIP141)
pub const VERIFY_WITNESS : c_uint = (1 << 11);

pub const VERIFY_ALL : c_uint = VERIFY_P2SH | VERIFY_DERSIG | VERIFY_NULLDUMMY |
    VERIFY_CHECKLOCKTIMEVERIFY | VERIFY_CHECKSEQUENCEVERIFY | VERIFY_WITNESS;

extern "C" {
    pub fn bitcoinconsensus_version() -> c_int;

    pub fn bitcoinconsensus_verify_script_with_amount(
        script_pubkey:  *const c_uchar,
        script_pubkeylen: c_uint,
        amount: uint64_t,
        tx_to: *const c_uchar,
        tx_tolen: c_uint,
        n_in: c_uint,
        flags: c_uint,
        err: *mut Error) -> c_int;
}

/// Compute flags for soft fork activation heights on the Bitcoin network
pub fn height_to_flags(height: u32) -> u32 {

    let mut flag = VERIFY_NONE;
    if height > 170059 {
        flag |= VERIFY_P2SH;
    }
    if height > 363724 {
        flag |= VERIFY_DERSIG;
    }
    if height > 388381 {
        flag |= VERIFY_CHECKLOCKTIMEVERIFY;
    }
    if height > 419328 {
        flag |= VERIFY_CHECKSEQUENCEVERIFY;
    }
    if height > 481824 {
        flag |= VERIFY_NULLDUMMY | VERIFY_WITNESS
    }
    flag as u32
}

/// Return libbitcoinconsenus version
pub fn version () -> u32 {
    unsafe { bitcoinconsensus_version() as u32 }
}

/// Verify a single spend (input) of a Bitcoin transaction.
/// # Arguments
///  * spend_output_script: a Bitcoin transaction output script to be spent, serialized in Bitcoin's on wire format
///  * amount: The spent output amount in satoshis
///  * spending_transaction: spending Bitcoin transaction, serialized in Bitcoin's on wire format
///  * input_index: index of the input within spending_transaction
/// # Returns
/// OK or Err. Note that amount will only be checked for Segwit transactions.
///
/// # Example
///
/// The (randomly choosen) Bitcoin transaction [aca326a724eda9a461c10a876534ecd5ae7b27f10f26c3862fb996f80ea2d45d](https://blockchain.info/tx/aca326a724eda9a461c10a876534ecd5ae7b27f10f26c3862fb996f80ea2d45d)
/// spends one input, that is the first output of [95da344585fcf2e5f7d6cbf2c3df2dcce84f9196f7a7bb901a43275cd6eb7c3f](https://blockchain.info/tx/95da344585fcf2e5f7d6cbf2c3df2dcce84f9196f7a7bb901a43275cd6eb7c3f) with a value of 630482530 satoshis
///
/// The spending transaction in wire format is:
///
/// `
/// spending = 02000000013f7cebd65c27431a90bba7f796914fe8cc2ddfc3f2cbd6f7e5f2fc854534da95000000006b483045022100de1ac3bcdfb0332207c4a91f3832bd2c2915840165f876ab47c5f8996b971c3602201c6c053d750fadde599e6f5c4e1963df0f01fc0d97815e8157e3d59fe09ca30d012103699b464d1d8bc9e47d4fb1cdaa89a1c5783d68363c4dbc4b524ed3d857148617feffffff02836d3c01000000001976a914fc25d6d5c94003bf5b0c7b640a248e2c637fcfb088ac7ada8202000000001976a914fbed3d9b11183209a57999d54d59f67c019e756c88ac6acb0700
/// `
///
/// The script of the first output of the spent transaction is:
///
/// `
/// spent = 1976a9144bfbaf6afb76cc5771bc6404810d1cc041a6933988ac
/// `
///
/// The (pseudo code) call:
///
/// `
/// verify(spent, 630482530, spending, 0)
/// `

should return OK(())
pub fn verify (spent_output: &[u8], amount: u64, spending_transaction: &[u8], input_index: usize) -> Result<(), Error> {
    verify_with_flags (spent_output, amount, spending_transaction, input_index, VERIFY_ALL)
}

/// Same as verify but with flags that turn past soft fork features on or off
pub fn verify_with_flags (spent_output_script: &[u8], amount: u64, spending_transaction: &[u8], input_index: usize, flags: u32) -> Result<(), Error> {
    unsafe {
        let mut error = Error::ERR_SCRIPT;

        let ret = bitcoinconsensus_verify_script_with_amount(
            spent_output_script.as_ptr(),
            spent_output_script.len() as c_uint,
            amount as uint64_t,
            spending_transaction.as_ptr(),
            spending_transaction.len() as c_uint,
            input_index as c_uint,
            flags as c_uint,
            &mut error
        );
        if ret != 1 {
            Err(error)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rustc_serialize as serialize;
    extern crate secp256k1;

    use super::verify;
    use super::Error;
    use self::serialize::hex::FromHex;

    #[test]
    fn bitcoinconsensus_test() {
        // a random old-style transaction from the blockchain
        verify_test (
            "1976a9144bfbaf6afb76cc5771bc6404810d1cc041a6933988ac",
            "02000000013f7cebd65c27431a90bba7f796914fe8cc2ddfc3f2cbd6f7e5f2fc854534da95000000006b483045022100de1ac3bcdfb0332207c4a91f3832bd2c2915840165f876ab47c5f8996b971c3602201c6c053d750fadde599e6f5c4e1963df0f01fc0d97815e8157e3d59fe09ca30d012103699b464d1d8bc9e47d4fb1cdaa89a1c5783d68363c4dbc4b524ed3d857148617feffffff02836d3c01000000001976a914fc25d6d5c94003bf5b0c7b640a248e2c637fcfb088ac7ada8202000000001976a914fbed3d9b11183209a57999d54d59f67c019e756c88ac6acb0700",
            0, 0
        ).unwrap();
    }

    fn verify_test (spent : &str, spending :&str, amount :u64, input: usize) -> Result<(),Error> {
        verify (spent.from_hex().unwrap().as_slice(), 630482530, spending.from_hex().unwrap().as_slice(), 0)
    }
}
