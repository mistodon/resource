# resource

[![Build Status](https://travis-ci.org/mistodon/resource.svg?branch=master)](https://travis-ci.org/mistodon/resource)
[![Crates.io](https://img.shields.io/crates/v/resource.svg)](https://crates.io/crates/resource)
[![Docs.rs](https://docs.rs/resource/badge.svg)](https://docs.rs/resource/0.6.0/resource/)

The `resource` crate contains macros for statically including assets in release mode, but dynamically loading them in debug mode.

This is primarily intended for games, allowing you to both avoid file IO in release builds and dynamically reload assets in debug mode.

You can change the default behaviour, in debug or release mode, by using the `force-static` and `force-dynamic` features.

When resources are included statically, they are constant in memory and are included in the final binary executable. This allows you to avoid packaging individual files with the released application.

When resources are included dynamically, they are loaded from file at runtime, and can therefore be switched and updated while the app runs.

## Usage

```toml
[dependencies]
resource = "0.6.0"
```

### Basic usage

```rust
use resource::{resource, resource_str};

let text = resource_str!("assets/text_asset.txt");
println!("Text is: {}", text);

let bytes = resource!("assets/binary_asset.bin");
println!("Binary data is: {:?}", bytes);

let (a, b, c) = resource_str!(("a.txt", "b.txt", "c.txt"));
println!("Contents of the three files are: `{}`, `{}`, `{}`");

let decoded_images = resource!(["a.png", "b.png", "c.png"], |image: &[u8]| decode(image));
```

### Reloading

```rust
use resource::resource_str;

let mut message = resource_str!("message.txt");

loop {
    println!("Hello: {}", message.as_ref());

    // Wait one second
    std::thread::sleep(std::time::Duration::from_secs(5));

    // You can edit the contents of message.txt

    // Reload the message so the new version is printed next iteration
    message.reload_if_changed();
}
```
