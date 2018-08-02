# resource

The `resource` crate contains macros for statically including assets in release mode, but dynamically loading them in debug mode.

This is primarily intended for games, allowing you to both avoid file IO in release builds and dynamically reload assets in debug mode.

You can change the default behaviour, in debug or release mode, by using the `force-static` and `force-dynamic` features.

[![Build Status](https://travis-ci.org/Mistodon/resource.svg?branch=master)](https://travis-ci.org/Mistodon/resource)
[![Crates.io](https://img.shields.io/crates/v/resource.svg)](https://crates.io/crates/resource)
[![Docs.rs](https://docs.rs/resource/badge.svg)](https://docs.rs/resource/0.2.0/resource/)

## Usage

```toml
[dependencies]
resource = "~0.2.0"
```

```rust
#[macro_use]
extern crate resource;

let text = resource_str!("assets/text_asset.txt");
println!("Text is: {}", text);

let bytes = resource!("assets/binary_asset.bin");
println!("Binary data is: {:?}", bytes);

let (a, b, c) = resource_str!(("a.txt", "b.txt", "c.txt"));
println!("Contents of the three files are: `{}`, `{}`, `{}`");

let decoded_images = resource!(["a.png", "b.png", "c.png"], |image: &[u8]| decode(image));
```

## Internals

The `resource_str!` and `resource!` macros return [`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html) values - `Cow<'static, str>` and `Cow<'static, [u8]>` respectively.

If you're not familiar with the `Cow` type, what this means is that under the hood, they can be either a reference to some `const` data (in release mode) or some actual owned data on the heap (in debug mode).

You shouldn't have to care about this though because the above `Cow` types can deref to `&str` and `&[u8]` respectively. Just pass them by reference and treat them as strings/slices.
