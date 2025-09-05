use std::process::Command;

fn main() {
    Command::new("tailwindcss")
        .args([
            "-i",
            "src/css/tailwind.css",
            "-o",
            "compiled_assets/css/tailwind.css",
        ])
        .output()
        .expect("Failed to build tailwindcss");

    Command::new("bun")
        .args(["scripts/bundle-js.ts"])
        .output()
        .expect("failed to build JS");
}
