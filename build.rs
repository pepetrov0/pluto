use std::{fs, path::PathBuf};

fn compile_tailwind_styles() {
    #[cfg(target_os = "windows")]
    const TAILWIND_EXECUTABLE_PATH: &str = "target/tailwindcss.exe";

    #[cfg(not(target_os = "windows"))]
    const TAILWIND_EXECUTABLE_PATH: &str = "target/tailwindcss";

    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    const TAILWIND_EXECUTABLE_URL: &str = "https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.3/tailwindcss-linux-x64";

    // check if already exists - if not - extract
    if !PathBuf::from(TAILWIND_EXECUTABLE_PATH).exists() {
        // download
        let bytes = reqwest::blocking::get(TAILWIND_EXECUTABLE_URL)
            .unwrap()
            .error_for_status()
            .unwrap()
            .bytes()
            .unwrap()
            .to_vec();

        // write
        fs::write(TAILWIND_EXECUTABLE_PATH, bytes).unwrap();

        // update permissions
        #[cfg(not(target_os = "windows"))]
        {
            let mut perms = fs::metadata(TAILWIND_EXECUTABLE_PATH)
                .unwrap()
                .permissions();
            std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
            fs::set_permissions(TAILWIND_EXECUTABLE_PATH, perms).unwrap();
        }
    }

    // compile styles
    let output = std::process::Command::new(TAILWIND_EXECUTABLE_PATH)
        .args(["-i", "src/styles.css", "-o", "target/styles.css", "-m"])
        .output()
        .unwrap();
    if output.status.code().unwrap_or_default() != 0 {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("{}\n{}\n", stdout, stderr);
    }

    // output rerun directives
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=src/styles.css");
    println!("cargo:rerun-if-changed={TAILWIND_EXECUTABLE_PATH}");
    println!("cargo:rerun-if-changed=target/styles.css");

    fs::read_dir("src/web/_components").unwrap().for_each(|v| {
        println!(
            "cargo:rerun-if-changed={}",
            v.unwrap().path().to_string_lossy().to_string()
        );
    })
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    compile_tailwind_styles();
}
