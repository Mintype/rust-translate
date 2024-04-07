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

/// # Supported Languages
///
/// This module provides a list of supported language codes for translation.
///
/// ## Usage
///
/// Users can refer to the documentation to see the list of supported languages and their
/// corresponding language codes.
///
/// # Examples
/// ```
/// use rust_translate::{translate, translate_to_english, translate_from_english};
/// use rust_translate::supported_languages::get_languages;
/// 
/// fn main() {
///     let supported_languages = get_languages();
///     println!("Supported languages: {:?}", supported_languages);
/// }
/// ```
pub mod supported_languages {
    /// List of supported language codes.
    ///
    /// The following language codes are supported for translation:
    ///
    /// - `af`: Afrikaans
    /// - `sq`: Albanian
    /// - `am`: Amharic
    /// - `ar`: Arabic
    /// - `hy`: Armenian
    /// - `az`: Azerbaijani
    /// - `eu`: Basque
    /// - `be`: Belarusian
    /// - `bn`: Bengali
    /// - `bs`: Bosnian
    /// - `bg`: Bulgarian
    /// - `ca`: Catalan
    /// - `ceb`: Cebuano
    /// - `ny`: Chichewa
    /// - `zh`: Chinese (Simplified)
    /// - `zh-TW`: Chinese (Traditional)
    /// - `co`: Corsican
    /// - `hr`: Croatian
    /// - `cs`: Czech
    /// - `da`: Danish
    /// - `nl`: Dutch
    /// - `en`: English
    /// - `eo`: Esperanto
    /// - `et`: Estonian
    /// - `tl`: Filipino
    /// - `fi`: Finnish
    /// - `fr`: French
    /// - `fy`: Frisian
    /// - `gl`: Galician
    /// - `ka`: Georgian
    /// - `de`: German
    /// - `el`: Greek
    /// - `gu`: Gujarati
    /// - `ht`: Haitian Creole
    /// - `ha`: Hausa
    /// - `haw`: Hawaiian
    /// - `he`: Hebrew
    /// - `hi`: Hindi
    /// - `hmn`: Hmong
    /// - `hu`: Hungarian
    /// - `is`: Icelandic
    /// - `ig`: Igbo
    /// - `id`: Indonesian
    /// - `ga`: Irish
    /// - `it`: Italian
    /// - `ja`: Japanese
    /// - `jv`: Javanese
    /// - `kn`: Kannada
    /// - `kk`: Kazakh
    /// - `km`: Khmer
    /// - `rw`: Kinyarwanda
    /// - `ko`: Korean
    /// - `ku`: Kurdish (Kurmanji)
    /// - `ky`: Kyrgyz
    /// - `lo`: Lao
    /// - `la`: Latin
    /// - `lv`: Latvian
    /// - `lt`: Lithuanian
    /// - `lb`: Luxembourgish
    /// - `mk`: Macedonian
    /// - `mg`: Malagasy
    /// - `ms`: Malay
    /// - `ml`: Malayalam
    /// - `mt`: Maltese
    /// - `mi`: Maori
    /// - `mr`: Marathi
    /// - `mn`: Mongolian
    /// - `my`: Myanmar (Burmese)
    /// - `ne`: Nepali
    /// - `no`: Norwegian
    /// - `or`: Odia (Oriya)
    /// - `ps`: Pashto
    /// - `fa`: Persian
    /// - `pl`: Polish
    /// - `pt`: Portuguese
    /// - `pa`: Punjabi
    /// - `ro`: Romanian
    /// - `ru`: Russian
    /// - `sm`: Samoan
    /// - `gd`: Scots Gaelic
    /// - `sr`: Serbian
    /// - `st`: Sesotho
    /// - `sn`: Shona
    /// - `sd`: Sindhi
    /// - `si`: Sinhala
    /// - `sk`: Slovak
    /// - `sl`: Slovenian
    /// - `so`: Somali
    /// - `es`: Spanish
    /// - `su`: Sundanese
    /// - `sw`: Swahili
    /// - `sv`: Swedish
    /// - `tg`: Tajik
    /// - `ta`: Tamil
    /// - `te`: Telugu
    /// - `th`: Thai
    /// - `tr`: Turkish
    /// - `uk`: Ukrainian
    /// - `ur`: Urdu
    /// - `ug`: Uyghur
    /// - `uz`: Uzbek
    /// - `vi`: Vietnamese
    /// - `cy`: Welsh
    /// - `xh`: Xhosa
    /// - `yi`: Yiddish
    /// - `yo`: Yoruba
    /// - `zu`: Zulu
pub fn get_languages() -> Vec<&'static str> {
    vec![
        "af", "sq", "am", "ar", "hy", "az", "eu", "be", "bn", "bs", "bg", "ca", "ceb", "ny", "zh", "zh-TW",
        "co", "hr", "cs", "da", "nl", "en", "eo", "et", "tl", "fi", "fr", "fy", "gl", "ka", "de", "el", "gu",
        "ht", "ha", "haw", "he", "hi", "hmn", "hu", "is", "ig", "id", "ga", "it", "ja", "jv", "kn", "kk", "km",
        "rw", "ko", "ku", "ky", "lo", "la", "lv", "lt", "lb", "mk", "mg", "ms", "ml", "mt", "mi", "mr", "mn",
        "my", "ne", "no", "or", "ps", "fa", "pl", "pt", "pa", "ro", "ru", "sm", "gd", "sr", "st", "sn", "sd",
        "si", "sk", "sl", "so", "es", "su", "sw", "sv", "tg", "ta", "te", "th", "tr", "uk", "ur", "ug", "uz",
        "vi", "cy", "xh", "yi", "yo", "zu",
        ]
    }
}
