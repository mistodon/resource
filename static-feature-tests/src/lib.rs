#![cfg(test)]

#[macro_use]
extern crate resource;

#[test]
fn include_str_asset_static() {
    let expected = "This\nis\na\nstring\n";
    let included = resource_str!("files/str.txt");
    assert_eq!(included, expected);
}

#[test]
fn included_str_is_borrowed_static() {
    use std::borrow::Cow;

    let included = resource_str!("files/str.txt");
    match included {
        Cow::Owned(_) => panic!("Included string should be a borrowed const."),
        Cow::Borrowed(_) => (),
    }
}

#[test]
fn include_binary_asset_static() {
    let expected: &[u8] = &[48, 49, 50, 51, 52];
    let included = resource!("files/bytes.bin");
    assert_eq!(included, expected);
}

#[test]
fn included_bytes_are_borrowed_static() {
    use std::borrow::Cow;

    let included = resource!("files/bytes.bin");
    match included {
        Cow::Owned(_) => panic!("Included string should be a borrowed const."),
        Cow::Borrowed(_) => (),
    }
}
