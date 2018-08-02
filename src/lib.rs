//! This crate contains macros for statically including resources in release mode,
//! but dynamically loading them in debug mode.
//!
//! This is primarily intended for games, allowing you to both avoid file IO in
//! release builds and dynamically reload resources in debug mode.
//!
//! ```rust,ignore
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
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main () {
/// let (toml, lib) = resource_str!(("Cargo.toml", "src/lib.rs"));
/// let as_array = resource_str!(["Cargo.toml", "src/lib.rs"]);
/// assert_eq!(toml, as_array[0]);
/// assert_eq!(lib, as_array[1]);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main () {
/// let [toml, lib] = resource_str!(["Cargo.toml", "src/lib.rs"], str::to_uppercase);
/// assert!(toml.contains("STATIC_ASSETS"));
/// assert!(lib.contains("MACRO_RULES"));
/// # }
/// ```
#[macro_export]
macro_rules! resource_str {
    ([ $($filenames:tt),* $(,)* ], $load_fn:expr) => {
        [ $($load_fn(resource_str!($filenames).as_ref())),* ]
    };

    (( $($filenames:tt),* $(,)* ), $load_fn:expr) => {
        ( $($load_fn(resource_str!($filenames).as_ref())),* )
    };

    ([ $($filenames:tt),* $(,)* ]) => {
        [ $(resource_str!($filenames)),* ]
    };

    (( $($filenames:tt),* $(,)* )) => {
        ( $(resource_str!($filenames)),* )
    };

    ($filename:tt, $load_fn:expr) => {
        $load_fn(resource_str!($filename).as_ref())
    };

    ($filename:tt) => {{
        #[cfg(
            any(
                feature = "force-static", all(not(feature = "force-dynamic"), not(debug_assertions))
            )
        )]
        {
            const RESOURCE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename));
            let result: ::std::borrow::Cow<'static, str> = ::std::borrow::Cow::Borrowed(RESOURCE);

            result
        }

        #[cfg(any(feature = "force-dynamic", all(not(feature = "force-static"), debug_assertions)))]
        {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename);

            let resource = ::std::fs::read_to_string(path).expect(concat!(
                "Failed to load string from: ",
                env!("CARGO_MANIFEST_DIR"),
                "/",
                $filename
            ));

            let result: ::std::borrow::Cow<'static, str> = ::std::borrow::Cow::Owned(resource);

            result
        }
    }};
}

/// Load multiple binary resources statically in release mode, or dynamically in
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
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main () {
/// let (toml, lib) = resource!(("Cargo.toml", "src/lib.rs"));
/// let as_array = resource!(["Cargo.toml", "src/lib.rs"]);
/// assert_eq!(toml, as_array[0]);
/// assert_eq!(lib, as_array[1]);
/// # }
/// ```
///
/// ```rust
/// # #[macro_use]
/// # extern crate static_assets;
/// # fn main () {
/// let [toml] = resource!(["Cargo.toml"],
///     |bytes: &[u8]| bytes.to_ascii_uppercase());
/// assert_eq!(&toml[0..9], b"[PACKAGE]");
/// # }
/// ```
#[macro_export]
macro_rules! resource {
    ([ $($filenames:tt),* $(,)* ], $load_fn:expr) => {
        [ $($load_fn(resource!($filenames).as_ref())),* ]
    };

    (( $($filenames:tt),* $(,)* ), $load_fn:expr) => {
        ( $($load_fn(resource!($filenames).as_ref())),* )
    };

    ([ $($filenames:tt),* $(,)* ]) => {
        [ $(resource!($filenames)),* ]
    };

    (( $($filenames:tt),* $(,)* )) => {
        ( $(resource!($filenames)),* )
    };

    ($filename:tt, $load_fn:expr) => {
        $load_fn(resource!($filename).as_ref())
    };

    ($filename:tt) => {{
        #[cfg(
            any(
                feature = "force-static", all(not(feature = "force-dynamic"), not(debug_assertions))
            )
        )]
        {
            const RESOURCE: &[u8] =
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename));
            let result: ::std::borrow::Cow<'static, [u8]> = ::std::borrow::Cow::Borrowed(RESOURCE);

            result
        }

        #[cfg(any(feature = "force-dynamic", all(not(feature = "force-static"), debug_assertions)))]
        {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename);

            let resource = ::std::fs::read(path).expect(concat!(
                "Failed to load bytes from: ",
                env!("CARGO_MANIFEST_DIR"),
                "/",
                $filename
            ));

            let result: ::std::borrow::Cow<'static, [u8]> = ::std::borrow::Cow::Owned(resource);

            result
        }
    }};
}
