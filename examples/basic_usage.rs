use rust_translate::{translate, translate_to_english, translate_from_english};
use rust_translate::supported_languages::get_languages;
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

    // List the supported languages of the crate
    let supported_languages = get_languages();
    println!("Supported languages: {:?}", supported_languages);
}
