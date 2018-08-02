# static_assets

The `static_assets` crate contains macros for statically including assets in release mode, but dynamically loading them in debug mode.

This is primarily intended for games, allowing you to both avoid file IO in release builds and dynamically reload assets in debug mode.

[![Build Status](https://travis-ci.org/Mistodon/static_assets.svg?branch=master)](https://travis-ci.org/Mistodon/static_assets)
[![Crates.io](https://img.shields.io/crates/v/static_assets.svg)](https://crates.io/crates/static_assets)

## Usage

```toml
[dependencies]
static_assets = "~0.2.0"
```

```rust
#[macro_use]
extern crate static_assets;

let text = asset_str!("assets/text_asset.txt");
println!("Text is: {}", text);

let bytes = asset_bytes!("assets/binary_asset.bin");
println!("Binary data is: {:?}", bytes);

let (a, b, c) = asset_str!(("a.txt", "b.txt", "c.txt"));
println!("Contents of the three files are: `{}`, `{}`, `{}`");

let decoded_images = asset_bytes!(["a.png", "b.png", "c.png"], |image: &[u8]| decode(image));
```

## Internals

The `asset_str!` and `asset_bytes!` macros return [`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html) values - `Cow<'static, str>` and `Cow<'static, [u8]>` respectively.

If you're not familiar with the `Cow` type, what this means is that under the hood, they can be either a reference to some `const` data (in release mode) or some actual owned data on the heap (in debug mode).

You shouldn't have to care about this though because the above `Cow` types can deref to `&str` and `&[u8]` respectively. Just pass them by reference and treat them as strings/slices.
