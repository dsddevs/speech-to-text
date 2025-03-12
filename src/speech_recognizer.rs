use slice::from_raw_parts;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::slice;
use std::time::Instant;
use vosk::{CompleteResult, DecodingState, Model, Recognizer};
use DecodingState::{Finalized, Running};

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
    pub fn recognize_audio_file(&self, audio_file_path: &str) -> Result<(), Box<dyn Error>> {
        let start_time = Instant::now();
        println!("SP Started: {}", audio_file_path);

        // vosk model
        let model = Model::new(&self.model_path).expect("Error: model loader failure");

        // vosk recognizer
        let mut recognizer =
            Recognizer::new(&model, self.sample_rate).expect("Error: Recognizer failure");

        recognizer.set_max_alternatives(1);
        recognizer.set_words(false);
        recognizer.set_partial_words(false);

        //audio file reader
        let mut file = File::open(audio_file_path)?;
        let file_size = file.metadata()?.len();
        println!("Audio file size: {} byte", file_size);

        // audio file process in chunks (to save memory)
        const CHUNK_SIZE: usize = 8192 * 2;
        let mut buffer = vec![0u8; CHUNK_SIZE];

        let mut processed = 0;
        while let Ok(n) = file.read(&mut buffer) {
            if n == 0 {
                break;
            }

            processed += n;

            // data conversion
            let samples = self.as_i16_slice(&buffer[0..n]);

            //data feeding
            match recognizer.accept_waveform(samples) {
                Ok(Running) => {
                    // Process in parts (%)
                    if processed % (CHUNK_SIZE * 10) == 0 {
                        println!(
                            "PROCESS: {:.1}%",
                            (processed as f64 / file_size as f64) * 100.0
                        );
                    }
                }
                Ok(Finalized) => {
                    // Process completed fragment
                }
                Err(e) => eprintln!("Error: in the process: {}", e),
                _ => {}
            }
        }

        // Final result
        let complete_result = recognizer.final_result();

        // Text name
        let audio_filename = Path::new(audio_file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown_audio");

        let text_filename = format!("text_{}", audio_filename)
            .replace(".wav", ".txt")
            .replace(".mp3", ".txt");

        let mut create_text_file = File::create(&text_filename)?;

        // Text extraction
        let text = match complete_result {
            CompleteResult::Single(single_result) => single_result.text.to_owned(),
            CompleteResult::Multiple(multi_result) => multi_result
                .alternatives
                .first()
                .map(|a| a.text)
                .unwrap_or_default()
                .to_owned(),
        };

        writeln!(create_text_file, "{}", text)?;

        let duration = start_time.elapsed();
        println!(
            "SP Ended: {:.2} seconds",
            duration.as_secs_f64()
        );
        println!("Success: text file created - {}", text_filename);

        Ok(())
    }

    //CONVERSION FUNCTION (slice u8 -> slice i16)
    fn as_i16_slice<'a>(&self, buffer: &'a [u8]) -> &'a [i16] {
        assert_eq!(
            buffer.len() % 2,
            0,
            "Error: Buffer size must be divisible by 2"
        );
        unsafe { from_raw_parts(buffer.as_ptr() as *const i16, buffer.len() / 2) }
    }
}
