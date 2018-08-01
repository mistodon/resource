//! This crate contains macros for statically including assets in release mode,
//! but dynamically loading them in debug mode.
//!
//! This is primarily intended for games, allowing you to both avoid file IO in
//! release builds and dynamically reload assets in debug mode.
//!
//! ```rust,ignore
//! // Include text
//! let readme_text = asset_str!("README.md");
//!
//! // Include bytes
//! let logo_bytes = asset_bytes!("assets/logo.png");
//!
//! // Load multiple strings
//! let translations = load_strings!(["english.txt", "italiano.txt"]);
//!
//! // Load and process multiple binary assets
//! let (light_texture, dark_texture) = load_bytes!(
//!     ("assets/light.png", "assets/dark.png"),
//!     Texture::decode);
//! ```

/// Load a text asset statically in release mode, or dynamically in debug.
///
/// The filename is relative to the root of your crate.
///
/// If you wish to override the static or dynamic behaviour, you can use the
/// `force-static` or `force-dynamic` features.
///
/// # Panics
///
/// When dynamically including, this will panic if the file does not exist. When
/// statically including, this will be a compile error and will never panic.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main() {
/// let toml = asset_str!("Cargo.toml");
/// assert!(toml.contains("[package]"));
/// # }
/// ```
#[macro_export]
macro_rules! asset_str {
    ($filename:tt) => {{
        #[cfg(
            any(
                feature = "force-static", all(not(feature = "force-dynamic"), not(debug_assertions))
            )
        )]
        {
            const ASSET: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename));
            let result: ::std::borrow::Cow<'static, str> = ::std::borrow::Cow::Borrowed(ASSET);

            result
        }

        #[cfg(any(feature = "force-dynamic", all(not(feature = "force-static"), debug_assertions)))]
        {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename);

            let asset = ::std::fs::read_to_string(path).expect(concat!(
                "Failed to load string from: ",
                env!("CARGO_MANIFEST_DIR"),
                "/",
                $filename
            ));

            let result: ::std::borrow::Cow<'static, str> = ::std::borrow::Cow::Owned(asset);

            result
        }
    }};
}

/// Load a binary asset statically in release mode, or dynamically in debug.
///
/// The filename is relative to the root of your crate.
///
/// If you wish to override the static or dynamic behaviour, you can use the
/// `force-static` or `force-dynamic` features.
///
/// # Panics
///
/// When dynamically including, this will panic if the file does not exist. When
/// statically including, this will be a compile error and will never panic.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main() {
/// let toml_bytes = asset_bytes!("Cargo.toml");
/// assert_eq!(&toml_bytes[0..9], b"[package]")
/// # }
/// ```
#[macro_export]
macro_rules! asset_bytes {
    ($filename:tt) => {{
        #[cfg(
            any(
                feature = "force-static", all(not(feature = "force-dynamic"), not(debug_assertions))
            )
        )]
        {
            const ASSET: &[u8] =
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename));
            let result: ::std::borrow::Cow<'static, [u8]> = ::std::borrow::Cow::Borrowed(ASSET);

            result
        }

        #[cfg(any(feature = "force-dynamic", all(not(feature = "force-static"), debug_assertions)))]
        {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename);

            let asset = ::std::fs::read(path).expect(concat!(
                "Failed to load bytes from: ",
                env!("CARGO_MANIFEST_DIR"),
                "/",
                $filename
            ));

            let result: ::std::borrow::Cow<'static, [u8]> = ::std::borrow::Cow::Owned(asset);

            result
        }
    }};
}

/// Load multiple text assets statically in release mode, or dynamically in debug.
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
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main () {
/// let (toml, lib) = load_strings!(("Cargo.toml", "src/lib.rs"));
/// let as_array = load_strings!(["Cargo.toml", "src/lib.rs"]);
/// assert_eq!(toml, as_array[0]);
/// assert_eq!(lib, as_array[1]);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main () {
/// let [toml, lib] = load_strings!(["Cargo.toml", "src/lib.rs"], str::to_uppercase);
/// assert!(toml.contains("STATIC_ASSETS"));
/// assert!(lib.contains("MACRO_RULES"));
/// # }
/// ```
#[macro_export]
macro_rules! load_strings {
    ([ $($filenames:tt),* $(,)* ], $load_fn:expr) => {
        [ $($load_fn(asset_str!($filenames).as_ref())),* ]
    };

    (( $($filenames:tt),* $(,)* ), $load_fn:expr) => {
        ( $($load_fn(asset_str!($filenames).as_ref())),* )
    };

    ([ $($filenames:tt),* $(,)* ]) => {
        [ $(asset_str!($filenames)),* ]
    };

    (( $($filenames:tt),* $(,)* )) => {
        ( $(asset_str!($filenames)),* )
    };
}

/// Load multiple binary assets statically in release mode, or dynamically in debug.
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
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main () {
/// let (toml, lib) = load_bytes!(("Cargo.toml", "src/lib.rs"));
/// let as_array = load_bytes!(["Cargo.toml", "src/lib.rs"]);
/// assert_eq!(toml, as_array[0]);
/// assert_eq!(lib, as_array[1]);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main () {
/// let [toml] = load_bytes!(["Cargo.toml"],
///     |bytes: &[u8]| bytes.to_ascii_uppercase());
/// assert_eq!(&toml[0..9], b"[PACKAGE]");
/// # }
/// ```
#[macro_export]
macro_rules! load_bytes {
    ([ $($filenames:tt),* $(,)* ], $load_fn:expr) => {
        [ $($load_fn(asset_bytes!($filenames).as_ref())),* ]
    };

    (( $($filenames:tt),* $(,)* ), $load_fn:expr) => {
        ( $($load_fn(asset_bytes!($filenames).as_ref())),* )
    };

    ([ $($filenames:tt),* $(,)* ]) => {
        [ $(asset_bytes!($filenames)),* ]
    };

    (( $($filenames:tt),* $(,)* )) => {
        ( $(asset_bytes!($filenames)),* )
    };
}
