#[macro_export]
macro_rules! asset_str
{
    ($filename: tt) => {
        {
            #[cfg(any(feature = "force-static", all(not(feature = "force-dynamic"), not(debug_assertions))))]
            {
                const ASSET: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename));
                let asset: ::std::borrow::Cow<'static, str> = ::std::borrow::Cow::Borrowed(ASSET);
                asset
            }

            #[cfg(any(feature = "force-dynamic", all(not(feature = "force-static"), debug_assertions)))]
            {
                let asset = $crate::load_str(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename));
                let asset: ::std::borrow::Cow<'static, str> = ::std::borrow::Cow::Owned(asset);
                asset
            }
        }
    }
}


#[macro_export]
macro_rules! asset_bytes
{
    ($filename: tt) => {
        {
            #[cfg(any(feature = "force-static", all(not(feature = "force-dynamic"), not(debug_assertions))))]
            {
                const ASSET: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename));
                let asset: ::std::borrow::Cow<'static, [u8]> = ::std::borrow::Cow::Borrowed(ASSET);
                asset
            }

            #[cfg(any(feature = "force-dynamic", all(not(feature = "force-static"), debug_assertions)))]
            {
                let asset = $crate::load_bytes(concat!(env!("CARGO_MANIFEST_DIR"), "/", $filename));
                let asset: ::std::borrow::Cow<'static, [u8]> = ::std::borrow::Cow::Owned(asset);
                asset
            }
        }
    }
}


#[cfg(any(feature = "force-dynamic", all(not(feature = "force-static"), debug_assertions)))]
#[inline]
pub fn load_str(filepath: &str) -> String
{
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(filepath).unwrap_or_else(|e| panic!("Failed to open file `{}`:\n{}", filepath, e));
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap_or_else(|e| panic!("Failed to read from `{}`:\n{}", filepath, e));
    buffer
}


#[cfg(any(feature = "force-dynamic", all(not(feature = "force-static"), debug_assertions)))]
#[inline]
pub fn load_bytes(filepath: &str) -> Vec<u8>
{
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(filepath).unwrap_or_else(|e| panic!("Failed to open file `{}`:\n{}", filepath, e));
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap_or_else(|e| panic!("Failed to read from `{}`:\n{}", filepath, e));
    buffer
}

