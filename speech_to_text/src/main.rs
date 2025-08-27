use crate::speech_recognizer::SpeechRecognizer;
use io::{stdin, stdout};
use std::error::Error;
use std::io::Write;
use std::{env, io, process};

mod audio_processor;
mod file_handler;
mod speech_recognizer;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let model_path = &args[1];
        let audio_path = &args[2];

        println!("The model is used: {}", model_path);
        println!("Processing audio file: {}", audio_path);

        run_recognition(model_path, audio_path)?;
    } else {

        println!("=== Speech recognition system ===");

        let model_path = get_user_input("Enter the path to the VOSK model: ")?;
        let audio_path = get_user_input("Enter the path to the audio file: ")?;

        run_recognition(&model_path, &audio_path)?;
    }

    Ok(())
}

fn run_recognition(model_path: &str, audio_path: &str) -> Result<(), Box<dyn Error>> {

    if !std::path::Path::new(model_path).exists() {
        eprintln!("Error: Model not found at path: {}", model_path);
        process::exit(1);
    }

    if !std::path::Path::new(audio_path).exists() {
        eprintln!("Error: Audio file not found at path: {}", audio_path);
        process::exit(1);
    }

    let recognizer = SpeechRecognizer::new(model_path, 16000.0);
    recognizer.recognize_audio_file(audio_path)?;

    println!("Recognition completed successfully!");
    Ok(())
}

fn get_user_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", prompt);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
