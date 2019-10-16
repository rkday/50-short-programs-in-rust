use std::collections::HashSet;

fn main() {
    let mut hs = HashSet::new();
    hs.insert("shinji");
    hs.insert("asuka");
    hs.insert("rei");
    hs.insert("misato");
    hs.insert("ritsuko");
    println!("{:?}", hs);
}
