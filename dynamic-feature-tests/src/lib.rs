#![cfg(test)]

#[macro_use]
extern crate resource;

#[test]
fn include_str_asset_dynamic() {
    let expected = "This\nis\na\nstring\n";
    let included = resource_str!("files/str.txt");
    assert_eq!(included, expected);
}

#[test]
fn included_str_is_owned_dynamic() {
    use std::borrow::Cow;

    let included = resource_str!("files/str.txt");
    match included {
        Cow::Borrowed(_) => panic!("Included string should be owned dynamically."),
        Cow::Owned(_) => (),
    }
}

#[test]
fn include_binary_asset_dynamic() {
    let expected: &[u8] = &[48, 49, 50, 51, 52];
    let included = resource!("files/bytes.bin");
    assert_eq!(included, expected);
}

#[test]
fn included_bytes_are_owned_dynamic() {
    use std::borrow::Cow;

    let included = resource!("files/bytes.bin");
    match included {
        Cow::Borrowed(_) => panic!("Included bytes should be owned dynamically."),
        Cow::Owned(_) => (),
    }
}
