//! **jmdict-rs** is a simple and lightweight Rust library
//! that provides an interface for accessing the **JMdict** Japanese language dictionary.
//! Instead of using the original XML files, this library utilizes preprocessed JSON files from the
//! [jmdict-simplified][jmdict-simplified] project.
//!
//! In order to reduce the crate size and to provide up-to-date entries,
//! language data is downloaded at build time from the latest release of **jmdict-simplified**.
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file
//!
//! ```toml
//! [dependencies]
//! jmdict-rs = "0.1.2"
//! ```
//!
//! # Usage
//!
//! ```rust
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let search = "楽勝";
//!     let entries = jmdict_rs::entries();
//!
//!     let entry = entries.iter().find(|e| {
//!         e.kanji.iter().any(|k| k.text == search)
//!     }).unwrap();
//!
//!     let texts: Vec<String> = entry.sense.iter().map(|s|
//!         s.gloss.iter().map(|g|
//!             g.text.clone()
//!         )
//!     ).flatten().collect();
//!     eprintln!("Search result = {:#?}", texts.iter().next().unwrap());
//!
//!     Ok(())
//! }
//! ```
//!
//! # License
//!
//! ## JMdict
//!
//! The origin of **JMdict** in the form of a JSON file is the repository [jmdict-simplified][jmdict-simplified].
//! In view of this, the said file is subject to the same license as its original source, namely **JMdict.xml**,
//! which is the intellectual property of the Electronic Dictionary Research and Development Group.
//! See [EDRDG License][EDRDG-license]
//!
//! ## Other files
//!
//! Source code and the rest of the files in this project are licensed under [Apache License, Version 2.0][Apache-2.0].
//!
//! [jmdict-simplified]: https://github.com/scriptin/jmdict-simplified
//! [EDRDG-license]: http://www.edrdg.org/edrdg/licence.html
//! [Apache-2.0]: http://www.apache.org/licenses/LICENSE-2.0

pub use crate::domain::{Entry, Gender, Gloss, GlossType, JMdict, Kana, Kanji, LanguageSource, Sense};

pub mod domain;

static JM_DICT_STR: &'static str = include_str!(concat!(env!("OUT_DIR"), "/jmdict-eng.json"));

fn load_jm_dict() -> JMdict {
    serde_json::from_str(JM_DICT_STR).unwrap()
}

pub fn entries() -> Vec<Entry> {
    load_jm_dict().entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_load_jm_dict() {
        assert_ne!(entries().len(), 0);
    }

    #[test]
    fn should_contain_specific_entry() {
        let search = "じゃが芋";
        let entries = entries();

        let entry = entries.iter().find(|e| {
            e.kanji.iter().any(|k| k.text == search)
        }).unwrap();

        let texts: Vec<String> = entry.sense.iter().map(|s|
            s.gloss.iter().map(|g|
                g.text.clone()
            )
        ).flatten().collect();

        assert!(texts.contains(&"potato (Solanum tuberosum)".to_string()))
    }
}
