//! This crate contains macros for statically including resources in release mode,
//! but dynamically loading them in debug mode.
//!
//! This is primarily intended for games, allowing you to both avoid file IO in
//! release builds and dynamically reload resources in debug mode.
//!
//! You can change the default behaviour, in debug or release mode, by using the
//! `force-static` and `force-dynamic` features.
//!
//! ```rust,ignore
//! use resource::{resource, resource_str};
//!
//! // Include text
//! let readme_text = resource_str!("README.md");
//!
//! // Include bytes
//! let logo_bytes = resource!("assets/logo.png");
//!
//! // Load multiple strings
//! let translations = resource_str!(["english.txt", "italiano.txt"]);
//!
//! // Load and process multiple binary resources
//! let (light_texture, dark_texture) = resource!(
//!     ("assets/light.png", "assets/dark.png"),
//!     Texture::decode);
//! ```

#[cfg(all(feature = "force-static", feature = "force-dynamic"))]
compile_error!("resource: Cannot enable both the force-static and force-dynamic features.");

pub use self::wrapper::Resource;

use std::path::Path;

pub trait ReadFromFile {
    fn read_from_file(path: &Path) -> Self;
}

impl ReadFromFile for String {
    fn read_from_file(path: &Path) -> String {
        std::fs::read_to_string(path)
            .map_err(|e| eprintln!("Failed to read `{}` as string: {}", path.display(), e))
            .unwrap()
    }
}

impl ReadFromFile for Vec<u8> {
    fn read_from_file(path: &Path) -> Vec<u8> {
        std::fs::read(path)
            .map_err(|e| eprintln!("Failed to read `{}` as bytes: {}", path.display(), e))
            .unwrap()
    }
}

#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
mod wrapper {
    use std::{
        borrow::{Cow, ToOwned},
        convert::AsRef,
        ops::Deref,
        path::{Path, PathBuf},
        time::SystemTime,
    };

    use crate::ReadFromFile;

    pub struct Resource<B>(<B as ToOwned>::Owned, PathBuf, SystemTime)
    where
        B: 'static + ToOwned + ?Sized;

    impl<B> Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
        B::Owned: ReadFromFile,
    {
        #[doc(hidden)]
        pub fn from_file(path: &str) -> Self {
            let path = PathBuf::from(path);
            let data = B::Owned::read_from_file(&path);
            let modified = Self::modified(&path).unwrap_or(SystemTime::UNIX_EPOCH);

            Resource(data, path, modified)
        }

        fn modified(path: &Path) -> Option<SystemTime> {
            std::fs::metadata(&path)
                .and_then(|metadata| metadata.modified())
                .ok()
        }

        pub fn changed(&self) -> bool {
            let modified = Self::modified(&self.1);
            modified.is_some() && modified != Some(self.2)
        }

        pub fn reload_if_changed(&mut self) {
            if self.changed() {
                self.reload();
            }
        }

        pub fn reload(&mut self) {
            let data = B::Owned::read_from_file(&self.1);
            let modified = Self::modified(&self.1).unwrap_or(SystemTime::UNIX_EPOCH);
            self.0 = data;
            self.2 = modified;
        }
    }

    impl<B> AsRef<B> for Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
        B::Owned: AsRef<B>,
    {
        fn as_ref(&self) -> &B {
            self.0.as_ref()
        }
    }

    impl<B> Deref for Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
        B::Owned: AsRef<B>,
    {
        type Target = B;

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl<B> Into<Cow<'static, B>> for Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
    {
        fn into(self) -> Cow<'static, B> {
            Cow::Owned(self.0)
        }
    }

    impl<B> Clone for Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
        B::Owned: Clone,
    {
        fn clone(&self) -> Self {
            Resource(self.0.clone(), self.1.clone(), self.2)
        }
    }
}

#[cfg(any(
    feature = "force-static",
    all(not(feature = "force-dynamic"), not(debug_assertions))
))]
mod wrapper {
    use std::{
        borrow::{Cow, ToOwned},
        convert::AsRef,
        ops::Deref,
    };

    use crate::ReadFromFile;

    pub struct Resource<B>(&'static B)
    where
        B: 'static + ToOwned + ?Sized;

    impl<B> Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
        B::Owned: ReadFromFile,
    {
        #[doc(hidden)]
        pub fn from_data(data: &'static B) -> Self {
            Resource(data)
        }

        pub fn changed(&self) -> bool {
            false
        }

        pub fn reload_if_changed(&mut self) {}

        pub fn reload(&mut self) {}
    }

    impl<B> AsRef<B> for Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
        B::Owned: AsRef<B>,
    {
        fn as_ref(&self) -> &B {
            self.0
        }
    }

    impl<B> Deref for Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
        B::Owned: AsRef<B>,
    {
        type Target = B;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    impl<B> Into<Cow<'static, B>> for Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
    {
        fn into(self) -> Cow<'static, B> {
            Cow::Borrowed(self.0)
        }
    }

    impl<B> Clone for Resource<B>
    where
        B: 'static + ToOwned + ?Sized,
        B::Owned: Clone,
    {
        fn clone(&self) -> Self {
            Resource(self.0)
        }
    }
}

/// Load text resources statically in release mode, or dynamically in debug.
///
/// The filenames are relative to the root of your crate.
///
/// If you wish to override the static or dynamic behaviour, you can use the
/// `force-static` or `force-dynamic` features.
///
/// This macro optionally takes a function which can be used to transform the
/// contents of each file on load.
///
/// # Panics
///
/// When dynamically including, this will panic if any file does not exist. When
/// statically including, this will be a compile error and will never panic.
///
/// # Examples
///
/// Load a single text file:
///
/// ```rust
/// use resource::resource_str;
///
/// let toml = resource_str!("Cargo.toml");
/// assert!(toml.contains("[package]"));
/// ```
///
/// Load an array or tuple of text files:
///
/// ```rust
/// use resource::resource_str;
///
/// let (toml, lib) = resource_str!(("Cargo.toml", "src/lib.rs"));
/// let as_array = resource_str!(["Cargo.toml", "src/lib.rs"]);
///
/// assert_eq!(toml.as_ref(), as_array[0].as_ref());
/// assert_eq!(lib.as_ref(), as_array[1].as_ref());
/// ```
///
/// Load multiple text files and apply a transformation to each one:
///
/// ```rust
/// use resource::resource_str;
///
/// let [toml, lib] = resource_str!(["Cargo.toml", "src/lib.rs"], str::to_uppercase);
///
/// assert!(toml.contains("RESOURCE"));
/// assert!(lib.contains("MACRO_RULES"));
/// ```
#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
#[macro_export]
macro_rules! resource_str {
    ([ $($filenames:tt),* $(,)* ], $load_fn:expr) => {
        [ $(resource_str!($filenames, $load_fn)),* ]
    };

    (( $($filenames:tt),* $(,)* ), $load_fn:expr) => {
        ( $(resource_str!($filenames, $load_fn)),* )
    };

    ([ $($filenames:tt),* $(,)* ]) => {
        [ $(resource_str!($filenames)),* ]
    };

    (( $($filenames:tt),* $(,)* )) => {
        ( $(resource_str!($filenames)),* )
    };

    ($filename:tt, $load_fn:expr) => {
        $load_fn(
            <$crate::Resource<str> as std::convert::AsRef<str>>::as_ref(&resource_str!($filename))
        )
    };

    ($filename:tt) => {
        $crate::Resource::<str>::from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename))
    };
}

#[cfg(any(
    feature = "force-static",
    all(not(feature = "force-dynamic"), not(debug_assertions))
))]
#[macro_export]
macro_rules! resource_str {
    ([ $($filenames:tt),* $(,)* ], $load_fn:expr) => {
        [ $(resource_str!($filenames, $load_fn)),* ]
    };

    (( $($filenames:tt),* $(,)* ), $load_fn:expr) => {
        ( $(resource_str!($filenames, $load_fn)),* )
    };

    ([ $($filenames:tt),* $(,)* ]) => {
        [ $(resource_str!($filenames)),* ]
    };

    (( $($filenames:tt),* $(,)* )) => {
        ( $(resource_str!($filenames)),* )
    };

    ($filename:tt, $load_fn:expr) => {
        $load_fn(
            <$crate::Resource<str> as std::convert::AsRef<str>>::as_ref(&resource_str!($filename))
        )
    };

    ($filename:tt) => {
        $crate::Resource::<str>::from_data(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename)))
    };
}

/// Load binary resources statically in release mode, or dynamically in
/// debug.
///
/// The filenames are relative to the root of your crate.
///
/// If you wish to override the static or dynamic behaviour, you can use the
/// `force-static` or `force-dynamic` features.
///
/// This macro optionally takes a function which can be used to transform the
/// contents of each file on load.
///
/// # Panics
///
/// When dynamically including, this will panic if any file does not exist. When
/// statically including, this will be a compile error and will never panic.
///
/// # Examples
///
/// Load a single binary file:
///
/// ```rust
/// use resource::resource;
///
/// let toml = resource!("Cargo.toml");
/// assert_eq!(&toml[0..9], b"[package]");
/// ```
///
/// Load an array or tuple of binary files:
///
/// ```rust
/// use resource::resource;
///
/// let (toml, lib) = resource!(("Cargo.toml", "src/lib.rs"));
/// let as_array = resource!(["Cargo.toml", "src/lib.rs"]);
///
/// assert_eq!(toml.as_ref(), as_array[0].as_ref());
/// assert_eq!(lib.as_ref(), as_array[1].as_ref());
/// ```
///
/// Load binary files and apply a transformation to each one:
///
/// ```rust
/// use resource::resource;
///
/// let [toml, lib] = resource!(["Cargo.toml", "src/lib.rs"],
///     |bytes: &[u8]| bytes.to_ascii_uppercase());
///
/// assert_eq!(&toml[0..9], b"[PACKAGE]");
/// assert_eq!(&lib[0..4], b"//! ");
/// ```
#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
#[macro_export]
macro_rules! resource {
    ([ $($filenames:tt),* $(,)* ], $load_fn:expr) => {
        [ $(resource!($filenames, $load_fn)),* ]
    };

    (( $($filenames:tt),* $(,)* ), $load_fn:expr) => {
        ( $(resource!($filenames, $load_fn)),* )
    };

    ([ $($filenames:tt),* $(,)* ]) => {
        [ $(resource!($filenames)),* ]
    };

    (( $($filenames:tt),* $(,)* )) => {
        ( $(resource!($filenames)),* )
    };

    ($filename:tt, $load_fn:expr) => {
        $load_fn(
            <$crate::Resource<[u8]> as std::convert::AsRef<[u8]>>::as_ref(&resource!($filename))
        )
    };

    ($filename:tt) => {
        $crate::Resource::<[u8]>::from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename))
    };
}

#[cfg(any(
    feature = "force-static",
    all(not(feature = "force-dynamic"), not(debug_assertions))
))]
#[macro_export]
macro_rules! resource {
    ([ $($filenames:tt),* $(,)* ], $load_fn:expr) => {
        [ $(resource!($filenames, $load_fn)),* ]
    };

    (( $($filenames:tt),* $(,)* ), $load_fn:expr) => {
        ( $(resource!($filenames, $load_fn)),* )
    };

    ([ $($filenames:tt),* $(,)* ]) => {
        [ $(resource!($filenames)),* ]
    };

    (( $($filenames:tt),* $(,)* )) => {
        ( $(resource!($filenames)),* )
    };

    ($filename:tt, $load_fn:expr) => {
        $load_fn(
            <$crate::Resource<[u8]> as std::convert::AsRef<[u8]>>::as_ref(&resource!($filename))
        )
    };

    ($filename:tt) => {
        $crate::Resource::<[u8]>::from_data(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename)))
    };
}

#[cfg(test)]
mod single_file_transform_tests {
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
}

#[cfg(test)]
mod multi_file_tests {
    fn rev_string(string: &str) -> String {
        string.chars().rev().collect()
    }

    fn rev_bytes(bytes: &[u8]) -> Vec<u8> {
        bytes.iter().cloned().rev().collect()
    }

    #[test]
    fn load_array_of_multiple_strings() {
        let [a, b, c] = resource_str!([
            "tests/string_a.txt",
            "tests/string_b.txt",
            "tests/string_c.txt",
        ]);

        assert_eq!([&*a, &*b, &*c], ["String A\n", "String B\n", "String C\n"]);
    }

    #[test]
    fn load_tuple_of_multiple_strings() {
        let (a, b, c) = resource_str!((
            "tests/string_a.txt",
            "tests/string_b.txt",
            "tests/string_c.txt",
        ));

        assert_eq!(&*a, "String A\n");
        assert_eq!(&*b, "String B\n");
        assert_eq!(&*c, "String C\n");
    }

    #[test]
    fn load_array_of_multiple_bytes() {
        let [a, b, c] = resource!([
            "tests/bytes_a.bin",
            "tests/bytes_b.bin",
            "tests/bytes_c.bin",
        ]);

        assert_eq!(
            [&*a, &*b, &*c],
            [
                b"Bytes A".as_ref(),
                b"Bytes B".as_ref(),
                b"Bytes C".as_ref()
            ]
        );
    }

    #[test]
    fn load_tuple_of_multiple_bytes() {
        let (a, b, c) = resource!((
            "tests/bytes_a.bin",
            "tests/bytes_b.bin",
            "tests/bytes_c.bin",
        ));

        assert_eq!(&*a, b"Bytes A".as_ref());
        assert_eq!(&*b, b"Bytes B".as_ref());
        assert_eq!(&*c, b"Bytes C".as_ref());
    }

    #[test]
    fn load_with_fn_array_of_multiple_strings() {
        let [a, b, c] = resource_str!(
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
        let (a, b, c) = resource_str!(
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
        let [a, b, c] = resource!(
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
        let (a, b, c) = resource!(
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
}

#[cfg(test)]
#[cfg(any(
    feature = "force-dynamic",
    all(not(feature = "force-static"), debug_assertions)
))]
mod dynamic_tests {
    use super::*;

    use std::borrow::Cow;

    #[test]
    fn str_dynamic() {
        match resource_str!("tests/str.txt").into() {
            Cow::Owned(ref s) if s == "This\nis\na\nstring\n" => (),
            _ => panic!("Expected owned string!"),
        }
    }

    #[test]
    fn bytes_dynamic() {
        match resource!("tests/bytes.bin").into() {
            Cow::Owned(ref s) if s == &[48, 49, 50, 51, 52] => (),
            _ => panic!("Expected owned bytes!"),
        }
    }
}

#[cfg(test)]
#[cfg(any(
    feature = "force-static",
    all(not(feature = "force-dynamic"), not(debug_assertions))
))]
mod static_tests {
    use super::*;

    use std::borrow::Cow;

    #[test]
    fn str_static() {
        match resource_str!("tests/str.txt").into() {
            Cow::Borrowed(s) if s == "This\nis\na\nstring\n" => (),
            _ => panic!("Expected borrowed string!"),
        }
    }

    #[test]
    fn bytes_static() {
        match resource!("tests/bytes.bin").into() {
            Cow::Borrowed(s) if s == &[48, 49, 50, 51, 52] => (),
            _ => panic!("Expected borrowed bytes!"),
        }
    }
}
