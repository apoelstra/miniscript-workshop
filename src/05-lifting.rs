
use miniscript::{Descriptor, DummyKey, Miniscript};
use miniscript::policy::Liftable;

use std::str::FromStr;

fn main() {
    // Things to try:
    //    1. Notice that in abstract policies, all pubkeys are pubkeyhashes. Why?
    //    2. Find the `all_attribute_tests` test in the Miniscript source
    //       and copy some examples out of there, and similarly play around.
    //    3. Find the documentation for `miniscript::policy::semantic::Policy`
    //       and try using the methods there. In particular `n_keys`,
    //       `minimum_n_keys` and `at_age`. Try with policies that have
    //       thresholds and/or timelocks.
    //    4. Discuss what functionality is missing and why it might be missing.
    //       Intractable to compute? Would result in combinatorial explosion?
    //       API complexity? Or just overlooked? Discuss how to overcome these
    //       problems. See also PR #70
    //    5. In particular, discuss the task of checking equivalence of
    //       checking "is necessary" and "is sufficient" for sets of keys.
    //       How are these tasks related to satisfaction/fee estimation?
    //       How do they relate to malleability concerns?
    let descriptor = Descriptor::<DummyKey>::from_str(
        "wsh(c:pk())",
    )
    .unwrap();

    let ms = Miniscript::<DummyKey>::from_str(
        "c:pk()",
    )
    .unwrap();

    let policy = ms.lift();

    println!("Descriptor: {}", descriptor);
    println!("Miniscript: {}", ms);
    println!("Policy: {}", policy);
}

