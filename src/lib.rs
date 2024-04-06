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
/// let translated_text = translate("Bonjour", "fr", "en").await.unwrap();
/// assert_eq!(translated_text, "Hello");
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
/// let translated_text = translate_to_english("Bonjour").await.unwrap();
/// assert_eq!(translated_text, "Hello");
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
/// let translated_text = translate_from_english("Hello", "fr").await.unwrap();
/// assert_eq!(translated_text, "Bonjour");
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