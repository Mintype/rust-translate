# rust-translation

A simple Rust crate for text translation between languages.

## Overview

`rust-translation` is a lightweight and easy-to-use Rust crate that simplifies the process of translating text into different languages using the Google Translate service.

## Features

- Seamless translation of text into multiple languages.
- Support for translating to and from English.
- Simple and intuitive API.

## Installation

Add `rust-translation` to your `Cargo.toml` file:

```toml
[dependencies]
rust-translation = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Usage

```rust
use rust_translate::{translate, translate_to_english, translate_from_english};

#[tokio::main]
async fn main() {
    // Translate text from any language to any other language
    let translated_text = translate("Bonjour le monde!", "fr", "en").await.unwrap();
    println!("Translated text: {}", translated_text);

    // Translate text to English
    let english_text = translate_to_english("Bonjour le monde!").await.unwrap();
    println!("Translated to English: {}", english_text);

    // Translate text from English to any other language
    let spanish_text = translate_from_english("Hello, world!", "es").await.unwrap();
    println!("Translated to Spanish: {}", spanish_text);
}
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
