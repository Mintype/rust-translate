use reqwest;
use serde_json::Value;

pub fn translate(text: &str, from: &str, to: &str) -> String {
    // placeholder
    text.to_string()    
}

pub fn translate_to_english(text: &str) -> String {
    // placeholder
    text.to_string()
}

pub async fn translate_from_english(text: &str, to: &str) -> Result<String, Box<dyn std::error::Error>>  {
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl={}&dt=t&q={}",
        to, text
    );

    let response = reqwest::get(&url).await?.text().await?;
    let translated_text: String = serde_json::from_str::<Value>(&response)?[0][0][0].as_str().unwrap().to_string();

    Ok(translated_text)
}