use std::{collections::BTreeMap, fs};

fn forbidden() {
    let _ = fs::read("state");
    let _ = BTreeMap::<String, String>::new();
}
