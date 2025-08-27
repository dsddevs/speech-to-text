use crate::audio_processor::AudioProcessor;
use crate::file_handler::FileHandler;
use std::error::Error;
use vosk::{Model, Recognizer};

pub struct SpeechRecognizer {
    pub model_path: String,
    pub sample_rate: f32,
}

impl SpeechRecognizer {
    pub fn new(model_path: &str, sample_rate: f32) -> Self {
        SpeechRecognizer {
            model_path: model_path.to_string(),
            sample_rate,
        }
    }

    pub fn recognize_audio_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        println!("START: Audio file recognition: {}", file_path);
        let recognizer = self.create_recognizer()?;
        let text = AudioProcessor::process_audio_file(file_path, recognizer)?;
        FileHandler::save_text_result(file_path, &text)?;
        println!("END: Audio file recognition: {}", file_path);
        Ok(())
    }

    fn create_recognizer(&self) -> Result<Recognizer, Box<dyn Error>> {
        let model = Model::new(&self.model_path).expect("Error: Model is not loaded!");

        let mut recognizer =
            Recognizer::new(&model, self.sample_rate).expect("Error: Recognizer is not created");

        recognizer.set_max_alternatives(1);
        recognizer.set_words(false);
        recognizer.set_partial_words(false);

        Ok(recognizer)
    }
}
