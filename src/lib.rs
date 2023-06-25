use crate::domain::{Entry, JMdict};
use crate::download::download_jm_dict;

pub mod domain;
mod download;

fn load_jm_dict() -> JMdict {
    let data = download_jm_dict().unwrap();

    serde_json::from_str(&data).unwrap()
}

pub fn jm_dict() -> JMdict {
    load_jm_dict()
}

pub fn entries() -> Vec<Entry> {
    load_jm_dict().entries
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref ENTRIES: Vec<Entry> = entries();
    }

    #[test]
    fn should_load_jm_dict() {
        assert_ne!(ENTRIES.len(), 0);
    }

    #[test]
    fn should_contain_specific_entry() {
        let search = "じゃが芋";

        let entry = ENTRIES.iter().find(|e| {
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
