use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=.env");
    println!("cargo::rerun-if-changed=src/js");
    println!("cargo::rerun-if-changed=src/css");
    println!("cargo::rerun-if-changed=templates");
    let tailwind = Command::new("tailwindcss")
        .args([
            "-i",
            "src/css/tailwind.css",
            "-o",
            "compiled_assets/css/tailwind.css",
        ])
        .output()
        .expect("Failed to build tailwindcss");
    println!("cargo::warning=Tailwind Status: {}", tailwind.status);
    let bun = Command::new("bun")
        .args(["scripts/bundle-js.ts"])
        .output()
        .expect("failed to build JS");
    println!("cargo::warning=Bun Status: {}", bun.status);
}
