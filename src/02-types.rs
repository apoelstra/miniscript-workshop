
use miniscript::{bitcoin, Descriptor, DummyKey, expression, Miniscript};

use std::str::FromStr;

fn main() {
    // Things to try:
    //    1. Remove the `c:`. What breaks?
    //       Notice that parsing `descriptor` succeeds. This is bug #71
    //    2. Try changing `c:` to `vc:`, or `cv:`. What happens?
    //    3. Change the `{}`s below to `{:?}` and observe the types of the debug output
    //    4. Find the `all_attribute_tests` test in the Miniscript source
    //       and copy some examples out of there, and similarly play around.
    let descriptor = Descriptor::<DummyKey>::from_str(
        "wsh(c:pk())",
    )
    .unwrap();

    let ms = Miniscript::<DummyKey>::from_str(
        "c:pk()",
    )
    .unwrap();

    // This is the "sanctioned" way to parse a Miniscript without doing the
    // typecheck. Notice it is really unergonomic and unintuitive. This is
    // because there is no reason to ever do this, except for illustration
    // purposes during a workshop.
    let expr = expression::Tree::from_str(
        "vc:pk()",
    )
    .unwrap();
    let ms_2: Miniscript::<DummyKey> = expression::FromTree::from_tree(&expr)
        .unwrap();

    println!("descriptor: {}", descriptor);
    println!("Miniscript 1: {}", ms);
    println!("Miniscript 2: {}", ms_2);
}

