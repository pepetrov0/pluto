use std::{fs, path::PathBuf, time::Duration};

#[cfg(target_os = "windows")]
const TAILWIND_EXECUTABLE_PATH: &str = "target/tailwindcss.exe";

#[cfg(not(target_os = "windows"))]
const TAILWIND_EXECUTABLE_PATH: &str = "target/tailwindcss";

fn rerun_if_dir_changed(path: PathBuf) {
    match path.is_dir() {
        true => {
            path.read_dir()
                .unwrap()
                .flatten()
                .for_each(|v| rerun_if_dir_changed(v.path()));
        }
        false => println!("cargo:rerun-if-changed={}", path.to_string_lossy()),
    }
}

fn check_and_download_tailwind() {
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    const TAILWIND_EXECUTABLE_URL: &str = "https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.3/tailwindcss-linux-x64";

    // check if already exists - if not - extract
    if !PathBuf::from(TAILWIND_EXECUTABLE_PATH).exists() {
        // download
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(300)) // 5 mins
            .build()
            .unwrap();
        let bytes = client
            .get(TAILWIND_EXECUTABLE_URL)
            .send()
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
}

fn compile_tailwind_styles() {
    // compile styles
    #[cfg(not(debug_assertions))]
    const ARGS: [&str; 5] = ["-i", "src/styles.css", "-o", "static/styles.css", "-m"];
    #[cfg(debug_assertions)]
    const ARGS: [&str; 4] = ["-i", "src/styles.css", "-o", "static/styles.css"];
    let output = std::process::Command::new(TAILWIND_EXECUTABLE_PATH)
        .args(&ARGS)
        .output()
        .unwrap();
    if output.status.code().unwrap_or_default() != 0 {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("{}\n{}\n", stdout, stderr);
    }
}

fn main() {
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=src/styles.css");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={TAILWIND_EXECUTABLE_PATH}");
    rerun_if_dir_changed("src/web".into());
    rerun_if_dir_changed("static".into());
    rerun_if_dir_changed("locales".into());

    check_and_download_tailwind();
    compile_tailwind_styles();
}
