use crate::domain::{Entry, JMdict};

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
