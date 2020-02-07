

use std::str::FromStr;

fn main() {
    // Things to try:
    //    1. Change the type to `String`. What breaks?
    //    2. Change the type to `miniscript::DummyKey`. What breaks?
    //    3. Change the key to an invalid EC key. What breaks?
    let my_descriptor = miniscript::Descriptor::<miniscript::bitcoin::PublicKey>::from_str(
//        "wsh(pk(020202020202020202020202020202020202020202020202020202020202020202))",
        "wsh(pk(040202020202020202020202020202020202020202020202020202020202020202\
                  0502020202020202020202020202020202020202020202020202020202020202))",
    )
    .unwrap();

    assert_eq!(
        format!("{:x}", my_descriptor.script_pubkey()),
        "0020a0a1a044f8d1e318caeba296ec10fe7c0939a59bc562dc013d39acbc724ded47"
    );

    assert_eq!(
        format!("{:x}", my_descriptor.witness_script()),
        "21020202020202020202020202020202020202020202020202020202020202020202"
    );

    // The `Display` output matches what we put into it
    println!("My descriptor: {}", my_descriptor);
    // The `Debug` output has a lot more information
    println!("My descriptor: {:?}", my_descriptor);
}

