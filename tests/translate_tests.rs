use rust_translate::{translate, translate_to_english, translate_from_english};

#[test]
fn test_translate() {
    // Prepare the input parameters
    let text = "Hello World";
    let from = "en";
    let to = "de";

    // Call the function under test
    let translated_text = translate(text, from, to);

    // Check if the result is as expected
    assert_eq!(translated_text, "Hallo Welt");
}

#[test]
fn test_translate_to_english() {
    // Prepare the input parameters
    let text = "Hallo Welt";

    // Call the function under test
    let translated_text = translate_to_english(text);

    // Check if the result is as expected
    assert_eq!(translated_text, "Hello World");
}

#[test]
fn test_translate_from_english() {
    // Prepare the input parameters
    let text = "Hello World";
    let to = "de";

    // Call the function under test
    let translated_text = translate_from_english(text, to);

    // Check if the result is as expected
    assert_eq!(translated_text, "Hallo Welt");
}

