use rust_translate::translate;
use std::io;

#[tokio::main]
async fn main() {
    
    // Print instructions
    println!("Please enter some text to translate to Mandarin Chinese:");

    // Create an infinite loop
    loop {
        // Create mutable variable for input
        let mut input = String::new();

        // Read line from console into input variable.
        io::stdin().read_line(&mut input)
            .expect("Failed to read line.");

        // Translate the input into `translated_text`
        let translated_text = translate(&input, "en", "zh").await.unwrap();
        
        // Print out translated text and instructions
        println!("Translated text: {}\nPlease enter some text to translate to Mandarin Chinese:", translated_text);
    }
}