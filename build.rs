fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jm_dict_str: String = jmdict_load::download_jm_dict()?;
    std::fs::write(&path_to_out_dir("jmdict-eng.json"), &jm_dict_str)?;

    Ok(())
}

fn path_to_out_dir(filename: &str) -> std::path::PathBuf {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    std::path::Path::new(&out_dir).join(filename)
}
