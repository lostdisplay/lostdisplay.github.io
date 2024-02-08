fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR env var is not set, is this running under cargo?");

    let target_dir =
        std::env::var("OUT_DIR").expect("OUT_DIR env var is not set, is this running under cargo?");

    //println!("cargo:rerun-if-changed={manifest_dir}/src");
    //println!("cargo:rerun-if-changed={manifest_dir}/build.rs");

    let output = std::process::Command::new("tailwindcss")
        .arg(format!("-i {}/src/input.css", manifest_dir))
        .arg(format!("-o {}/public/tailwind.css", manifest_dir))
        .output()
        .expect("Coult not run tailwind compiler. Is tailwindcss installed?");

    if !output.status.success() {
        println!(
            "tailwind stdout:\n{}",
            String::from_utf8(output.stdout).unwrap()
        )
    }
}
