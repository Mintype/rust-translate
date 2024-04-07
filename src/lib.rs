use reqwest;
use serde_json::Value;

/// Translates text from one language to another.
///
/// # Arguments
///
/// * `text` - The text to be translated.
/// * `from` - The language code of the source language.
/// * `to` - The language code of the target language.
///
/// # Examples
///
///
/// ```
/// use rust_translate::{translate, translate_to_english, translate_from_english};
///
/// #[tokio::main]
/// async fn main() {
///     // Translate text from any language to any other language
///     let translated_text = translate("Bonjour le monde!", "fr", "en").await.unwrap();
///     println!("Translated text: {}", translated_text);
/// }
/// ```
pub async fn translate(text: &str, from: &str, to: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        from, to, text
    );

    let response = reqwest::get(&url).await?.text().await?;
    let translated_text: String = serde_json::from_str::<Value>(&response)?[0][0][0].as_str().unwrap().to_string();

    Ok(translated_text)
}

/// Translates text to English from the detected language.
///
/// # Arguments
///
/// * `text` - The text to be translated.
///
/// # Examples
///
/// ```
/// use rust_translate::{translate, translate_to_english, translate_from_english};
///
/// #[tokio::main]
/// async fn main() {
///     // Translate text to English
///     let english_text = translate_to_english("Bonjour le monde!").await.unwrap();
///     println!("Translated to English: {}", english_text);
/// }
/// ```
pub async fn translate_to_english(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let from = "auto";
    let to = "en";

    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        from, to, text
    );

    let response = reqwest::get(&url).await?.text().await?;
    let translated_text: String = serde_json::from_str::<Value>(&response)?[0][0][0].as_str().unwrap().to_string();

    Ok(translated_text)
}

/// Translates text from English to the specified language.
///
/// # Arguments
///
/// * `text` - The text to be translated.
/// * `to` - The language code of the target language.
///
/// # Examples
///
/// ```
/// use rust_translate::{translate, translate_to_english, translate_from_english};
///
/// #[tokio::main]
/// async fn main() {
///     // Translate text from English to any other language
///     let spanish_text = translate_from_english("Hello, world!", "es").await.unwrap();
///     println!("Translated to Spanish: {}", spanish_text);
/// }
/// ```
pub async fn translate_from_english(text: &str, to: &str) -> Result<String, Box<dyn std::error::Error>>  {
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl={}&dt=t&q={}",
        to, text
    );

    let response = reqwest::get(&url).await?.text().await?;
    let translated_text: String = serde_json::from_str::<Value>(&response)?[0][0][0].as_str().unwrap().to_string();

    Ok(translated_text)
}