#[macro_use]
extern crate resource;

use std::borrow::Cow;

#[test]
#[cfg(debug_assertions)]
fn str_dynamic_in_debug_mode() {
    match resource_str!("tests/str.txt") {
        Cow::Owned(ref s) if s == "This\nis\na\nstring\n" => (),
        _ => panic!("Expected owned string!"),
    }
}

#[test]
#[cfg(not(debug_assertions))]
fn str_static_in_release_mode() {
    match resource_str!("tests/str.txt") {
        Cow::Borrowed(s) if s == "This\nis\na\nstring\n" => (),
        _ => panic!("Expected borrowed string!"),
    }
}

#[test]
#[cfg(debug_assertions)]
fn bytes_dynamic_in_debug_mode() {
    match resource!("tests/bytes.bin") {
        Cow::Owned(ref s) if s == &[48, 49, 50, 51, 52] => (),
        _ => panic!("Expected owned bytes!"),
    }
}

#[test]
#[cfg(not(debug_assertions))]
fn bytes_static_in_release_mode() {
    match resource!("tests/bytes.bin") {
        Cow::Borrowed(s) if s == &[48, 49, 50, 51, 52] => (),
        _ => panic!("Expected borrowed bytes!"),
    }
}
