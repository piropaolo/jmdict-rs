const EMPTY_JM_DICT: &'static str = r#"{"version":"no-data","languages":[],"commonOnly":false,"dictDate":"1970-01-01","dictRevisions":[],"tags":{},"words":[]}"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jm_dict_str: String = if cfg!(not(feature = "no-data")) {
        jmdict_load::download_jm_dict()?
    } else {
        EMPTY_JM_DICT.to_string()
    };
    std::fs::write(&path_to_out_dir("jmdict-eng.json"), &jm_dict_str)?;

    Ok(())
}

fn path_to_out_dir(filename: &str) -> std::path::PathBuf {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    std::path::Path::new(&out_dir).join(filename)
}
