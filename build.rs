use std::env;
use std::fs;

fn main() {
    // Detect if this is a release build
    let profile = env::var("PROFILE").unwrap();
    let binary_name = "site.exe"; // replace this with your actual binary name

    // Get the current directory
    let current_dir = env::current_dir().unwrap();
    let target_dir = match profile.as_str() {
        "release" => current_dir.join("target\\release"),
        _ => current_dir.join("target\\debug"),
    };

    // Construct the path to the final binary
    let final_binary = target_dir.join(binary_name);
    println!("cargo:rerun-if-changed={}", final_binary.display());

    // Print out the path for debugging purposes
    println!("cargo:warning=Final binary will be at: {:?}", final_binary);

    if final_binary.exists() {
        println!("cargo:warning=Binary found: {:?}", final_binary);
        println!("cargo:warning=Copying the binary to {:?}", current_dir.join(binary_name));
        fs::copy(final_binary, current_dir.join(binary_name)).unwrap();
    } else {
        println!("cargo:warning=Binary not found: {:?}", final_binary);
    }
}