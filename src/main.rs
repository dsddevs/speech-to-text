use crate::speech_recognizer::SpeechRecognizer;
use std::error::Error;
use vosk::set_log_level;
use vosk::LogLevel::Warn;

mod speech_recognizer;

fn main() -> Result<(), Box<dyn Error>> {

    set_log_level(Warn);

    let recognizer = SpeechRecognizer::new(
        "VOSK_KOREAN_MODEL_PATH", 16000.0
    );

    recognizer.recognize_audio_file("AUDIO_FILE_PATH")
        .expect("Error: While recognizing audio file");

    Ok(())

}