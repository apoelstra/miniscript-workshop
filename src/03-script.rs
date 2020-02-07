
use miniscript::{bitcoin, Miniscript};
use miniscript::bitcoin::hashes::hex;

fn main() {
    // Things to try:
    //    1. Mar one of the opcodes (e.g. the initial 63 (OP_IF).
    //    2. Mar some byte in one of the public keys.
    //    3. Change one of the 00's near the end to a 51 (push 0 → push 1).
    //    4. Find something on the blockchain.
    let script_bytes: Vec<u8> = hex::FromHex::from_hex(
        "6363829263522103d01115d548e7561b15c38f004d734633687cf4419620095bc5b0f\
         47070afe85a21025601570cb47f238d2b0286db4a990fa0f3ba28d1a319f5e7cf55c2\
         a2444da7cc52af0400046749b168670068670068"
    ).unwrap();
    let script = bitcoin::Script::from(script_bytes);
    let ms = Miniscript::<bitcoin::PublicKey>::parse(&script).unwrap();

    println!("Script: {}", script);
    println!("Miniscript: {}", ms);
}

