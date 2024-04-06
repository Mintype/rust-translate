use rust_translate::{translate, translate_to_english, translate_from_english};

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

#[test]
fn test_translate_to_english() {
    // Prepare the input parameters
    let text = "Hallo Welt";

    // Call the function under test
    let translated_text = translate_to_english(text);

    // Check if the result is as expected
    assert_eq!(translated_text, "Hello World");
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

