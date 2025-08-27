fn main() {
    println!("cargo:rustc-link-search=native=vosk-win64-0.3.45");

    // FOR WINDOWS:
    println!("cargo:rustc-link-lib=libvosk");

    // FOR LINUX:
    // println!("cargo:rustc-link-lib=dylib=vosk");
}

