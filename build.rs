fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR env var is not set, is this running under cargo?");

    let target_dir =
        std::env::var("OUT_DIR").expect("OUT_DIR env var is not set, is this running under cargo?");

    println!("cargo:rerun-if-changed={manifest_dir}/src");
    println!("cargo:rerun-if-changed={manifest_dir}/build.rs");

    let mut cmd = std::process::Command::new("npx");
    let cmd = cmd
        .arg("tailwindcss")
        .args(["-i", &format!("{}/src/input.css", manifest_dir)])
        .args(["-o", &format!("{}/public/tailwind.css", manifest_dir)]);

    let output = cmd
        .output()
        .expect("Coult not run tailwind compiler. Did you run `npm install`?");

    if !output.status.success() {
        let line = "========";
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();

        panic!("{line} tailwind failed \n -> manifest dir: {manifest_dir}\n command -> {cmd:?}.\nstdout:\n{stdout}\n\nstderr:\n{stderr}\n{line}",)
    }
}
