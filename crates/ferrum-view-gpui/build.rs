fn main() {
    #[cfg(target_os = "linux")]
    {
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            if let Some(workspace_dir) = std::path::Path::new(&manifest_dir).ancestors().nth(2) {
                let local_lib = workspace_dir.join("lib");
                if local_lib.exists() {
                    println!("cargo:rustc-link-search=native={}", local_lib.display());
                }
            }
        }
    }
}
