# Speech-to-Text Engine

## Overview
This project is a command-line tool for converting speech from audio files into text using the Vosk speech recognition engine. It processes large audio files in efficient chunks, converting raw audio data from `u8` to `i16` slices and leveraging the Vosk library to perform the speech-to-text conversion. The recognized text is then saved to an output text file.

## Technology Stack
- **Programming Language:** Rust
- **Speech Recognition:** Vosk Speech Recognition API (via the [`vosk` crate](https://crates.io/crates/vosk))
- **Core Dependencies:**
    - Rust Standard Library: `std::error::Error`, `std::fs::File`, `std::io::{Read, Write}`, `std::path::Path`, `std::slice`, and `std::time::Instant`
    - Vosk Model: Requires a pre-downloaded speech recognition model from [Vosk Models](https://alphacephei.com/vosk/models)

## Dependencies
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [Vosk Model Data](https://alphacephei.com/vosk/models) (download a model suitable for your language)
- The [`vosk` Rust crate](https://crates.io/crates/vosk) (add this to your `Cargo.toml`):
  ```toml
  [dependencies]
  vosk = "x.y.z"  # Replace x.y.z with the appropriate version
  ```

## Project Structure
- **SpeechRecognizer Struct:**  
  Contains methods for initializing the recognizer, processing audio files, and converting byte slices.
    - `new(model_path: &str, sample_rate: f32)`: Constructor to initialize the recognizer with the path to the Vosk model and the audio sample rate.
    - `recognize_audio_file(audio_file_path: &str)`: Reads the audio file in predefined chunks, converts each chunk for processing, and uses the Vosk API to recognize speech. The final text output is written to a file with a derived name.
    - `as_i16_slice(buffer: &[u8])`: Converts a slice of bytes to a slice of 16-bit integers. This is used to prepare audio data for the recognition process.

## Setup Instructions
Follow these steps to set up and run the project:

1. **Clone the Repository**
   ```bash
   git clone https://github.com/dsddevs/speech-to-text-rust.git
   cd speech-to-text-rust
   ```

2. **Install Rust and Cargo**
   If you haven’t installed Rust already, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

3. **Download the Vosk Model**
    - Visit the [Vosk Models](https://alphacephei.com/vosk/models) page.
    - Download a model that suits your language requirements.
    - Extract the model to a directory on your machine.

4. **Configure the Environment**
   Set the environment variable for the model path or update the application configuration to point to your model:
   ```bash
   export VOSK_MODEL_PATH="/path/to/your/model"
   ```

5. **Build the Project**
   Use Cargo to build the project in release mode:
   ```bash
   cargo build --release
   ```

6. **Run the Application**
   Execute the application by providing an audio file as an argument:
   ```bash
   cargo run --release -- /path/to/audio.wav
   ```
   The tool will process the provided audio file and create a corresponding text file containing the recognized speech.

## Workflow and Processing Details
- **File Processing:**  
  The audio file is read in chunks (using a chunk size defined in the code), ensuring efficient memory usage. Each chunk is converted from `u8` data into `i16` slices for feeding into the recognizer.

- **Speech Recognition:**  
  The Vosk recognizer handles the data in streaming mode, processing each chunk and collecting partial results. Once the processing is complete, it finalizes the recognition to produce the complete text output.

- **Output Generation:**  
  The final recognized text is saved to a text file whose name is derived from the original audio filename.

- **Logging and Performance:**  
  The program logs processing progress and computes the total time taken for the conversion, providing insights into the performance of the tool.

## Error Handling
- The application uses Rust's robust error handling (`Result` and `Box<dyn Error>`) to capture I/O errors, decoding issues, and other potential runtime problems.
- In case of errors during file read/write operations or recognition processing, appropriate error messages will be displayed.

## Future Enhancements
- Improving error messages and adding more comprehensive exception handling.
- Supporting real-time or streaming audio input.
- Extending functionality to handle various audio formats more gracefully.
- Adding configuration options for advanced tuning of the Vosk recognizer parameters.

## Contributing
Contributions are welcome! Please fork the repository, create a new branch for your changes, and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

## License
This project is licensed under the Apache License 2.0.
