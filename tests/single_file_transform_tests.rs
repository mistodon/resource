#[macro_use]
extern crate resource;

fn rev_string(string: &str) -> String {
    string.chars().rev().collect()
}

fn rev_bytes(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().cloned().rev().collect()
}

#[test]
fn transform_single_string() {
    let s = resource_str!("tests/str.txt", rev_string);
    assert_eq!(s, "\ngnirts\na\nsi\nsihT");
}

#[test]
fn transform_single_bytes() {
    let s = resource!("tests/bytes.bin", rev_bytes);
    assert_eq!(s, &[52, 51, 50, 49, 48]);
}
