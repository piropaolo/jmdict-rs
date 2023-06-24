use std::collections::HashMap;

use serde::{Deserialize, Serialize};

type Tag = String;
type Language = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JMdict {
    pub version: String,
    pub languages: Vec<Language>,
    pub common_only: bool,
    pub dict_date: String,
    pub dict_revisions: Vec<String>,
    pub tags: HashMap<Tag, String>,
    #[serde(rename = "words")]
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: String,
    pub kanji: Vec<Kanji>,
    pub kana: Vec<Kana>,
    pub sense: Vec<Sense>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kanji {
    pub common: bool,
    pub text: String,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kana {
    pub common: bool,
    pub text: String,
    pub tags: Vec<Tag>,
    pub applies_to_kanji: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sense {
    pub part_of_speech: Vec<Tag>,
    pub applies_to_kanji: Vec<String>,
    pub applies_to_kana: Vec<String>,
    //TODO related
    //TODO antonym
    pub field: Vec<Tag>,
    pub dialect: Vec<Tag>,
    pub misc: Vec<Tag>,
    pub info: Vec<String>,
    pub language_source: Vec<LanguageSource>,
    pub gloss: Vec<Gloss>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LanguageSource {
    pub lang: Language,
    pub full: bool,
    pub wasei: bool,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GlossType {
    Literal,
    Figurative,
    Explanation,
    Trademark,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gloss {
    pub lang: Language,
    pub gender: Option<Gender>,
    #[serde(rename = "type")]
    pub type_: Option<GlossType>,
    pub text: String,
}
