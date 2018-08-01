//! This crate contains macros for statically including assets in release mode,
//! but dynamically loading them in debug mode.
//!
//! This is primarily intended for games, allowing you to both avoid file IO in
//! release builds and dynamically reload assets in debug mode.

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
/// ```rust,ignore
/// let toml = asset_str!("Cargo.toml");
/// println!("{}", toml);
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
/// ```rust,ignore
/// let toml_bytes = asset_bytes!("Cargo.toml");
/// println!("{:?}", toml_bytes);
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
