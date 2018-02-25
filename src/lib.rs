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
#[derive(Clone, Debug,PartialEq)]
#[repr(C)]
pub enum Error {
    ERR_OK = 0,
    #[allow(dead_code)]
    ERR_TX_INDEX,
    #[allow(dead_code)]
    ERR_TX_SIZE_MISMATCH,
    #[allow(dead_code)]
    ERR_TX_DESERIALIZE,
    #[allow(dead_code)]
    ERR_AMOUNT_REQUIRED
}

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub const VERIFY_NONE : c_uint = 0;
#[allow(non_upper_case_globals)]
// evaluate P2SH (BIP16) subscripts
pub const VERIFY_P2SH : c_uint = (1 << 0);
#[allow(non_upper_case_globals)]
// enforce strict DER (BIP66) compliance
pub const VERIFY_DERSIG : c_uint = (1 << 2);
#[allow(non_upper_case_globals)]
// enforce NULLDUMMY (BIP147)
pub const VERIFY_NULLDUMMY : c_uint = (1 << 4);
#[allow(non_upper_case_globals)]
// enable CHECKLOCKTIMEVERIFY (BIP65)
pub const VERIFY_CHECKLOCKTIMEVERIFY : c_uint = (1 << 9);
#[allow(non_upper_case_globals)]
// enable CHECKSEQUENCEVERIFY (BIP112)
pub const VERIFY_CHECKSEQUENCEVERIFY : c_uint = (1 << 10);
#[allow(non_upper_case_globals)]
// enable WITNESS (BIP141)
pub const VERIFY_WITNESS : c_uint = (1 << 11);

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

/// soft fork activation heights on the Bitcoin network
pub fn height_to_flag (height: u32) -> u32 {

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

pub fn verify_spend (spent_output: &[u8], amount: u64, spending_transaction: &[u8], output_index: usize, flag: u32) -> Result<(), Error> {
    unsafe {
        let mut error = Error::ERR_OK;

        let ret = bitcoinconsensus_verify_script_with_amount(
            spent_output.as_ptr(), spent_output.len() as c_uint,
            amount as uint64_t,
            spending_transaction.as_ptr(),
            spending_transaction.len() as c_uint,
            output_index as c_uint,
            flag as c_uint,
            &mut error
        );
        if ret != 1 {
            Err(error)
        } else {
            Ok(())
        }
    }
}
