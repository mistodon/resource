#[macro_use]
extern crate static_assets;

fn rev_string(string: &str) -> String {
    string.chars().rev().collect()
}

fn rev_bytes(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().cloned().rev().collect()
}

#[test]
fn load_array_of_multiple_strings() {
    let [a, b, c] = load_strings!([
        "tests/string_a.txt",
        "tests/string_b.txt",
        "tests/string_c.txt",
    ]);

    assert_eq!([a, b, c], ["String A\n", "String B\n", "String C\n"]);
}

#[test]
fn load_tuple_of_multiple_strings() {
    let (a, b, c) = load_strings!((
        "tests/string_a.txt",
        "tests/string_b.txt",
        "tests/string_c.txt",
    ));

    assert_eq!(a, "String A\n");
    assert_eq!(b, "String B\n");
    assert_eq!(c, "String C\n");
}

#[test]
fn load_array_of_multiple_bytes() {
    let [a, b, c] = load_bytes!([
        "tests/bytes_a.bin",
        "tests/bytes_b.bin",
        "tests/bytes_c.bin",
    ]);

    assert_eq!(
        [a, b, c],
        [
            b"Bytes A".as_ref(),
            b"Bytes B".as_ref(),
            b"Bytes C".as_ref()
        ]
    );
}

#[test]
fn load_tuple_of_multiple_bytes() {
    let (a, b, c) = load_bytes!((
        "tests/bytes_a.bin",
        "tests/bytes_b.bin",
        "tests/bytes_c.bin",
    ));

    assert_eq!(a, b"Bytes A".as_ref());
    assert_eq!(b, b"Bytes B".as_ref());
    assert_eq!(c, b"Bytes C".as_ref());
}

#[test]
fn load_with_fn_array_of_multiple_strings() {
    let [a, b, c] = load_strings!(
        [
            "tests/string_a.txt",
            "tests/string_b.txt",
            "tests/string_c.txt",
        ],
        rev_string
    );

    assert_eq!([a, b, c], ["\nA gnirtS", "\nB gnirtS", "\nC gnirtS"]);
}

#[test]
fn load_with_fn_tuple_of_multiple_strings() {
    let (a, b, c) = load_strings!(
        (
            "tests/string_a.txt",
            "tests/string_b.txt",
            "tests/string_c.txt",
        ),
        rev_string
    );

    assert_eq!(a, "\nA gnirtS");
    assert_eq!(b, "\nB gnirtS");
    assert_eq!(c, "\nC gnirtS");
}

#[test]
fn load_with_fn_array_of_multiple_bytes() {
    let [a, b, c] = load_bytes!(
        [
            "tests/bytes_a.bin",
            "tests/bytes_b.bin",
            "tests/bytes_c.bin",
        ],
        rev_bytes
    );

    assert_eq!(
        [a, b, c],
        [
            b"A setyB".as_ref(),
            b"B setyB".as_ref(),
            b"C setyB".as_ref()
        ]
    );
}

#[test]
fn load_with_fn_tuple_of_multiple_bytes() {
    let (a, b, c) = load_bytes!(
        (
            "tests/bytes_a.bin",
            "tests/bytes_b.bin",
            "tests/bytes_c.bin",
        ),
        rev_bytes
    );

    assert_eq!(a, b"A setyB".as_ref());
    assert_eq!(b, b"B setyB".as_ref());
    assert_eq!(c, b"C setyB".as_ref());
}
