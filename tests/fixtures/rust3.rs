use std::collections::BTreeMap;

// This is a comment
fn main() {
    let x = 3;
    if x < 3 {
    } else if x == 4 {
    } else {
    }
    loop {}

    // This is a multiline comment
    // This is a multiline comment
    let map = BTreeMap::new();
    for x in map.iter() {}
    while true {
        break;
    }
    /*
    this is a big comment
    */
    map.insert("one\n", r"two\n");
}
