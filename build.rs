fn main() {
    println!("cargo:rustc-link-search=native=VOSK_DRIVER_PATH");
    println!("cargo:rustc-link-lib=libvosk");
}