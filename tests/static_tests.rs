#![cfg(any(feature = "force-static", all(not(feature = "force-dynamic"), not(debug_assertions))))]

#[macro_use]
extern crate resource;

#[test]
fn include_str_asset_static() {
    #[cfg(not(feature = "custom-path"))]
    let expected = "This\nis\na\nstring\n";

    #[cfg(feature = "custom-path")]
    let expected = "This is a custom path!\n";

    let included = resource_str!("tests/str.txt");
    assert_eq!(included, expected);
}

#[test]
fn included_str_is_borrowed_static() {
    use std::borrow::Cow;

    let included = resource_str!("tests/str.txt");
    match included {
        Cow::Owned(_) => panic!("Included string should be a borrowed const."),
        Cow::Borrowed(_) => (),
    }
}

#[test]
fn include_binary_asset_static() {
    #[cfg(not(feature = "custom-path"))]
    let expected: &[u8] = &[48, 49, 50, 51, 52];

    #[cfg(feature = "custom-path")]
    let expected: &[u8] = &[72, 101, 108, 108, 111];

    let included = resource!("tests/bytes.bin");
    assert_eq!(included, expected);
}

#[test]
fn included_bytes_are_borrowed_static() {
    use std::borrow::Cow;

    let included = resource!("tests/bytes.bin");
    match included {
        Cow::Owned(_) => panic!("Included string should be a borrowed const."),
        Cow::Borrowed(_) => (),
    }
}
