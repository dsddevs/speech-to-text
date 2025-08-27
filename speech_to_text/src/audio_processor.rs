use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::slice::from_raw_parts;
use vosk::{CompleteResult, DecodingState, Recognizer};

pub struct AudioProcessor;

impl AudioProcessor {
    const CHUNK_SIZE: usize = 8192 * 2;
    const PROGRESS_UPDATE_INTERVAL: usize = Self::CHUNK_SIZE * 10;

    pub fn process_audio_file(
        audio_file_path: &str,
        mut recognizer: Recognizer
    ) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(audio_file_path)?;
        let file_size = file.metadata()?.len();
        println!("Audio file size: {} bytes", file_size);

        let mut buffer = vec![0u8; Self::CHUNK_SIZE];
        let mut processed = 0;

        while let Ok(n) = file.read(&mut buffer) {
            if n == 0 {
                break;
            }

            processed += n;
            let samples = Self::convert_to_i16_samples(&buffer[0..n])?;

            Self::feed_audio_data(&mut recognizer, samples)?;
            Self::update_progress(processed, file_size);
        }

        Self::extract_final_text(recognizer)
    }

    fn convert_to_i16_samples(buffer: &[u8]) -> Result<&[i16], Box<dyn Error>> {
        if buffer.len() % 2 != 0 {
            return Err("The buffer size must be a multiple of 2".into());
        }

        Ok(unsafe {
            from_raw_parts(buffer.as_ptr() as *const i16, buffer.len() / 2)
        })
    }

    fn feed_audio_data(recognizer: &mut Recognizer, samples: &[i16]) -> Result<(), Box<dyn Error>> {
        match recognizer.accept_waveform(samples) {
            Ok(DecodingState::Running) => {
                Ok(())
            },
            Ok(DecodingState::Finalized) => {
                Ok(())
            },
            Ok(other_state) => {
                println!("Unexpected decoding state: {:?}", other_state);
                Ok(())
            },
            Err(e) => {
                Err(format!("Audio processing error: {}", e).into())
            }
        }
    }

    fn update_progress(processed: usize, file_size: u64) {
        if processed % Self::PROGRESS_UPDATE_INTERVAL == 0 {
            let progress = (processed as f64 / file_size as f64) * 100.0;
            println!("PROGRESS: {:.1}%", progress);
        }
    }

    fn extract_final_text(mut recognizer: Recognizer) -> Result<String, Box<dyn Error>> {
        let complete_result = recognizer.final_result();

        let text = match complete_result {
            CompleteResult::Single(result) => result.text,
            CompleteResult::Multiple(results) => {
                results
                    .alternatives
                    .first()
                    .map(|a| a.text)
                    .unwrap_or_default()
            }
        };

        Ok(text.to_owned())
    }
}