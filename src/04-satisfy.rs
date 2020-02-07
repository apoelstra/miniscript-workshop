
// Basically copied from Miniscript examples/sign_multisig.rs

use miniscript::{bitcoin, Descriptor};
use miniscript::bitcoin::secp256k1;  // note use of re-exported dependencies
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    // Data (skip down to "Things to try" for the real code
    let mut tx = bitcoin::Transaction {
        version: 2,
        lock_time: 0,
        input: vec![bitcoin::TxIn {
            previous_output: Default::default(),
            script_sig: bitcoin::Script::new(),
            sequence: 0xffffffff,
            witness: vec![],
        }],
        output: vec![bitcoin::TxOut {
            script_pubkey: bitcoin::Script::new(),
            value: 100_000_000,
        }],
    };

    let public_keys = vec![
        bitcoin::PublicKey::from_slice(&[2; 33]).expect("key 1"),
        bitcoin::PublicKey::from_slice(&[
            0x02,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]).expect("key 2"),
        bitcoin::PublicKey::from_slice(&[
            0x03,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]).expect("key 3"),
    ];
    let bitcoin_sig = (
        // copied at random off the blockchain; this is not actually a valid
        // signature for this transaction; Miniscript's default implementations
        // of the `Satisfier` trait do not verify signatures
        secp256k1::Signature::from_str(
            "3045\
             0221\
             00f7c3648c390d87578cd79c8016940aa8e3511c4104cb78daa8fb8e429375efc1\
             0220\
             531d75c136272f127a5dc14acc0722301cbddc222262934151f140da345af177",
        )
        .unwrap(),
        bitcoin::SigHashType::All,
    );





    // Things to try
    //    1. Try signing with 0 or 1 signatures. How does it fail?
    //    2. Try signing with a third signature. Check that it uses only two
    //    3. Try changing the miniscript (will need to disable the assertions)
    //       to require a absolute or relative timelock.
    //    4. Try changing the miniscript to use a regular threshold rather
    //       than thresh_m. Observe the script.

    let descriptor_str = format!(
        "wsh(thresh_m(2,{},{},{}))",
        public_keys[0], public_keys[1], public_keys[2],
    );

    // Descriptor for the output being spent
    let descriptor = Descriptor::<bitcoin::PublicKey>::from_str(&descriptor_str[..])
        .expect("parse descriptor string");

    // Observe the script properties, just for fun
    assert_eq!(
        format!("{:x}", descriptor.script_pubkey()),
        "00200ed49b334a12c37f3df8a2974ad91ff95029215a2b53f78155be737907f06163"
    );

    assert_eq!(
        format!("{:x}", descriptor.witness_script()),
        "52\
         21020202020202020202020202020202020202020202020202020202020202020202\
         21020102030405060708010203040506070801020304050607080000000000000000\
         21030102030405060708010203040506070801020304050607080000000000000000\
         53ae"
    );

    // Attempt to satisfy at age 0, height 0, with two signatures
    println!("Unsigned transaction: {:?}", tx);

    let original_txin = tx.input[0].clone();

    let mut sigs = HashMap::<bitcoin::PublicKey, miniscript::BitcoinSig>::new();
    sigs.insert(public_keys[0], bitcoin_sig);
    sigs.insert(public_keys[1], bitcoin_sig);
    assert!(descriptor.satisfy(&mut tx.input[0], &sigs).is_ok());
    assert_ne!(tx.input[0], original_txin);
    assert_eq!(tx.input[0].witness.len(), 4); // 0, sig, sig, witness script

    // Notice crappy witness-printing output. See discussion on #rust-bitcoin
    // about creating a `Witness` type which wraps `Vec<Vec<u8>>` and prints in hex
    println!("Signed transaction: {:?}", tx);
}

