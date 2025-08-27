use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct FileHandler;

impl FileHandler {
    pub fn save_text_result(
        audio_file_path: &str,
        text: &str
    ) -> Result<(), Box<dyn Error>> {
        let text_filename = Self::generate_text_filename(audio_file_path);
        let mut file = File::create(&text_filename)?;

        writeln!(file, "{}", text)?;

        println!("Successfully created text file: {}", text_filename);
        Ok(())
    }

    fn generate_text_filename(audio_file_path: &str) -> String {
        let path = Path::new(audio_file_path);

        let filename_without_ext = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown_audio");

        let parent_dir = path.parent()
            .map(|p| p.to_str().unwrap_or(""))
            .unwrap_or("");

        if parent_dir.is_empty() {
            format!("text_{}.txt", filename_without_ext)
        } else {
            format!("{}/text_{}.txt", parent_dir, filename_without_ext)
        }
    }
}