use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use CompleteResult::{Multiple, Single};
use DecodingState::{Finalized, Running};
use vosk::{CompleteResult, DecodingState, Model, Recognizer};

pub struct SpeechRecognizer {
    model_path: String,
    sample_rate: f32,
}

impl SpeechRecognizer {
    //Constructor
    pub fn new(model_path: &str, sample_rate: f32) -> Self {
        SpeechRecognizer {
            model_path: model_path.to_string(),
            sample_rate,
        }
    }

    // AUDIO FILE RECOGNIZER FUNCTION
    pub fn recognize_audio_file(
        &self,
        audio_file_path: &str,
    ) -> Result<(), Box<dyn Error>> {

        // speech model loader
        let model = Model::new(&self.model_path).expect("Error: model loader failure");

        // speech recognizer
        let mut recognizer =
            Recognizer::new(&model, self.sample_rate).expect("Error: Recognizer failure");


        //audio file reader
        let mut file = File::open(audio_file_path)?;


        // audio file buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // validation
        let samples = self.as_i16_slice(&buffer);
        match recognizer.accept_waveform(samples) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: Sound wave not accepted - {}", e);
            }
        }

        match recognizer.accept_waveform(samples) {
            Ok(Running) => {},
            Ok(Finalized) => {},
            Err(e) => eprintln!("Critical error: {}", e),
            _ => {}
        }

        //result
        let complete_result = recognizer.final_result();

        //audio file name getter
        let audio_filename = Path::new(audio_file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown_audio");

        // text file name setter
        let text_filename = format!("text_{}", audio_filename)
            .replace(".wav", ".text")
            .replace(".mp3", ".text");

        let mut create_text_file = File::create(&text_filename)?;

        let text = match complete_result {
            Single(single_result) => single_result.text.to_owned(),
            Multiple(multi_result) => {
                multi_result.alternatives.first()
                    .map(|a| a.text)
                    .unwrap_or_default()
                    .to_owned()
            }
        };
        writeln!(create_text_file, "{}", text)?;

        println!("Successfully created audio file {}", audio_file_path);

        Ok(())
    }

    //CONVERSATION FUNCTION (slice u8 -> slice i16)
    fn as_i16_slice<'a>(&self, buffer: &'a [u8]) -> &'a [i16] {
        assert_eq!(
            buffer.len() % 2,
            0,
            "Error: Buffer size must be divisible by 2"
        );
        unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const i16, buffer.len() / 2) }
    }
}

