use rust_translate::{translate, translate_to_english, translate_from_english};
use rust_translate::supported_languages::get_languages;

#[tokio::test]
async fn test_translate() {
    // Prepare the input parameters
    let text = "Hola Amigos";
    let from = "es";
    let to = "de";

    // Call the function under test
    let translated_text_result = translate(text, from, to).await;

    // Check if the result is as expected
    match translated_text_result {
        Ok(translated_text) => {
            assert_eq!(translated_text, "Hallo Freunde");
        },
        Err(err) => {
            // Print out the error message
            eprintln!("Error: {}", err);
            // Fail the test
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_translate_to_english() {
    // Prepare the input parameters
    let text = "Estoy comiendo queso.";

    // Call the function under test
    let translated_text_result = translate_to_english(text).await;

    // Check if the result is as expected
    match translated_text_result {
        Ok(translated_text) => {
            assert_eq!(translated_text, "I'm eating cheese.");
        },
        Err(err) => {
            // Print out the error message
            eprintln!("Error: {}", err);
            // Fail the test
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_translate_from_english() {
    // Prepare the input parameters
    let text = "Hello World";
    let to = "de";

    // Call the function under test
    let translated_text_result = translate_from_english(text, to).await;

    // Check if the result is as expected
    match translated_text_result {
        Ok(translated_text) => {
            assert_eq!(translated_text, "Hallo Welt");
        },
        Err(err) => {
            // Print out the error message
            eprintln!("Error: {}", err);
            // Fail the test
            assert!(false);
        }
    }
}

#[test]
fn test_supported_languages() {
    // Create a vector of the list of languages
    let languages = get_languages();

    // Assert that the number of languages is valid
    assert_eq!(languages.len(), 107); // Assuming there are 107 languages in the list

    // Assert that each of the languages below are in the list
    assert!(languages.contains(&"af")); // Afrikaans
    assert!(languages.contains(&"sq")); // Albanian
    assert!(languages.contains(&"am")); // Amharic
    assert!(languages.contains(&"ar")); // Arabic
    assert!(languages.contains(&"hy")); // Armenian
    assert!(languages.contains(&"az")); // Azerbaijani
    assert!(languages.contains(&"eu")); // Basque
    assert!(languages.contains(&"be")); // Belarusian
    assert!(languages.contains(&"bn")); // Bengali
    assert!(languages.contains(&"bs")); // Bosnian
    assert!(languages.contains(&"bg")); // Bulgarian
    assert!(languages.contains(&"ca")); // Catalan
    assert!(languages.contains(&"ceb")); // Cebuano
    assert!(languages.contains(&"ny")); // Chichewa
    assert!(languages.contains(&"zh")); // Chinese (Simplified)
    assert!(languages.contains(&"zh-TW")); // Chinese (Traditional)
    assert!(languages.contains(&"co")); // Corsican
    assert!(languages.contains(&"hr")); // Croatian
    assert!(languages.contains(&"cs")); // Czech
    assert!(languages.contains(&"da")); // Danish
    assert!(languages.contains(&"nl")); // Dutch
    assert!(languages.contains(&"en")); // English
    assert!(languages.contains(&"eo")); // Esperanto
    assert!(languages.contains(&"et")); // Estonian
    assert!(languages.contains(&"tl")); // Filipino
    assert!(languages.contains(&"fi")); // Finnish
    assert!(languages.contains(&"fr")); // French
    assert!(languages.contains(&"fy")); // Frisian
    assert!(languages.contains(&"gl")); // Galician
    assert!(languages.contains(&"ka")); // Georgian
    assert!(languages.contains(&"de")); // German
    assert!(languages.contains(&"el")); // Greek
    assert!(languages.contains(&"gu")); // Gujarati
    assert!(languages.contains(&"ht")); // Haitian Creole
    assert!(languages.contains(&"ha")); // Hausa
    assert!(languages.contains(&"haw")); // Hawaiian
    assert!(languages.contains(&"he")); // Hebrew
    assert!(languages.contains(&"hi")); // Hindi
    assert!(languages.contains(&"hmn")); // Hmong
    assert!(languages.contains(&"hu")); // Hungarian
    assert!(languages.contains(&"is")); // Icelandic
    assert!(languages.contains(&"ig")); // Igbo
    assert!(languages.contains(&"id")); // Indonesian
    assert!(languages.contains(&"ga")); // Irish
    assert!(languages.contains(&"it")); // Italian
    assert!(languages.contains(&"ja")); // Japanese
    assert!(languages.contains(&"jv")); // Javanese
    assert!(languages.contains(&"kn")); // Kannada
    assert!(languages.contains(&"kk")); // Kazakh
    assert!(languages.contains(&"km")); // Khmer
    assert!(languages.contains(&"rw")); // Kinyarwanda
    assert!(languages.contains(&"ko")); // Korean
    assert!(languages.contains(&"ku")); // Kurdish (Kurmanji)
    assert!(languages.contains(&"ky")); // Kyrgyz
    assert!(languages.contains(&"lo")); // Lao
    assert!(languages.contains(&"la")); // Latin
    assert!(languages.contains(&"lv")); // Latvian
    assert!(languages.contains(&"lt")); // Lithuanian
    assert!(languages.contains(&"lb")); // Luxembourgish
    assert!(languages.contains(&"mk")); // Macedonian
    assert!(languages.contains(&"mg")); // Malagasy
    assert!(languages.contains(&"ms")); // Malay
    assert!(languages.contains(&"ml")); // Malayalam
    assert!(languages.contains(&"mt")); // Maltese
    assert!(languages.contains(&"mi")); // Maori
    assert!(languages.contains(&"mr")); // Marathi
    assert!(languages.contains(&"mn")); // Mongolian
    assert!(languages.contains(&"my")); // Myanmar (Burmese)
    assert!(languages.contains(&"ne")); // Nepali
    assert!(languages.contains(&"no")); // Norwegian
    assert!(languages.contains(&"or")); // Odia (Oriya)
    assert!(languages.contains(&"ps")); // Pashto
    assert!(languages.contains(&"fa")); // Persian
    assert!(languages.contains(&"pl")); // Polish
    assert!(languages.contains(&"pt")); // Portuguese
    assert!(languages.contains(&"pa")); // Punjabi
    assert!(languages.contains(&"ro")); // Romanian
    assert!(languages.contains(&"ru")); // Russian
    assert!(languages.contains(&"sm")); // Samoan
    assert!(languages.contains(&"gd")); // Scots Gaelic
    assert!(languages.contains(&"sr")); // Serbian
    assert!(languages.contains(&"st")); // Sesotho
    assert!(languages.contains(&"sn")); // Shona
    assert!(languages.contains(&"sd")); // Sindhi
    assert!(languages.contains(&"si")); // Sinhala
    assert!(languages.contains(&"sk")); // Slovak
    assert!(languages.contains(&"sl")); // Slovenian
    assert!(languages.contains(&"so")); // Somali
    assert!(languages.contains(&"es")); // Spanish
    assert!(languages.contains(&"su")); // Sundanese
    assert!(languages.contains(&"sw")); // Swahili
    assert!(languages.contains(&"sv")); // Swedish
    assert!(languages.contains(&"tg")); // Tajik
    assert!(languages.contains(&"ta")); // Tamil
    assert!(languages.contains(&"te")); // Telugu
    assert!(languages.contains(&"th")); // Thai
    assert!(languages.contains(&"tr")); // Turkish
    assert!(languages.contains(&"uk")); // Ukrainian
    assert!(languages.contains(&"ur")); // Urdu
    assert!(languages.contains(&"ug")); // Uyghur
    assert!(languages.contains(&"uz")); // Uzbek
    assert!(languages.contains(&"vi")); // Vietnamese
    assert!(languages.contains(&"cy")); // Welsh
    assert!(languages.contains(&"xh")); // Xhosa
    assert!(languages.contains(&"yi")); // Yiddish
    assert!(languages.contains(&"yo")); // Yoruba
    assert!(languages.contains(&"zu")); // Zulu
}
