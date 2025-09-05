use std::process::Command;

fn main() {
    Command::new("tailwindcss")
        .args(["-i", "tailwind.css", "-o", "compiled_assets/tailwind.css"])
        .output()
        .expect("Failed to build tailwindcss");
}
