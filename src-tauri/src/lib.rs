use core::panic;
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Translations {
    translation: String,
    r#type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Phrases {
    phrase: String,
    translation: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Word {
    word: String,
    translations: Vec<Translations>,
    phrases: Option<Vec<Phrases>>,
}
struct Vocabularies {
    junior_highschool: Vec<Word>,
    highschool: Vec<Word>,
    cet4: Vec<Word>,
    cet6: Vec<Word>,
    graduate_school_examination: Vec<Word>,
    toefl: Vec<Word>,
}

impl Vocabularies {
    fn get_vocabulary(&self, vocabulary: &str) -> Vec<Word> {
        match vocabulary {
            "初中" => self.junior_highschool.clone(),
            "高中" => self.highschool.clone(),
            "CET4" => self.cet4.clone(),
            "CET6" => self.cet6.clone(),
            "考研" => self.graduate_school_examination.clone(),
            "托福" => self.toefl.clone(),
            _ => vec![],
        }
    }
}

static VOCABULARIES: Lazy<Vocabularies> = Lazy::new(|| Vocabularies {
    junior_highschool: serde_json::from_str(include_str!("./vocabularies/1-初中-顺序.json"))
        .unwrap(),
    highschool: serde_json::from_str(include_str!("./vocabularies/2-高中-顺序.json")).unwrap(),
    cet4: serde_json::from_str(include_str!("./vocabularies/3-CET4-顺序.json")).unwrap(),
    cet6: serde_json::from_str(include_str!("./vocabularies/4-CET6-顺序.json")).unwrap(),
    graduate_school_examination: serde_json::from_str(include_str!(
        "./vocabularies/5-考研-顺序.json"
    ))
    .unwrap(),
    toefl: serde_json::from_str(include_str!("./vocabularies/6-托福-顺序.json")).unwrap(),
});
static DATA_LOCATION: Lazy<PathBuf> = Lazy::new(|| {
    if cfg!(target_os = "windows") {
        PathBuf::from(std::env::var("APPDATA").expect("Could not found APP_DATA directory"))
            .join("word-book")
    } else if cfg!(target_os = "linux") {
        PathBuf::from(std::env::var("HOME").expect("Could not found home")).join(".word-book")
    } else if cfg!(target_os = "macos") {
        PathBuf::from("/Users/").join("word-book")
    } else if cfg!(target_os = "android") {
        PathBuf::from("")
    } else {
        panic!("Sorry, but this program don't support your system!")
    }
});

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_words,
            save_unskilled_words,
            load_unskilled_words_groups,
            load_unskilled_words,
            delete_unskilled_words_group,
            get_words_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_words_count(vocabulary: String) -> usize {
    VOCABULARIES.get_vocabulary(&vocabulary).len()
}

#[tauri::command]
fn get_words(vocabulary: String, start: usize, end: usize) -> Vec<Word> {
    let words = VOCABULARIES.get_vocabulary(&vocabulary);
    words[start..=end].to_vec()
}

#[tauri::command]
fn save_unskilled_words(words: Vec<Word>) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let directory = DATA_LOCATION.join("unskilled_words");
    let _ = std::fs::create_dir_all(&directory);
    let _ = fs::write(
        directory.join(time.to_string()),
        serde_json::to_string_pretty(&words).unwrap(),
    );
}

#[tauri::command]
fn load_unskilled_words_groups() -> Vec<u128> {
    let directory = DATA_LOCATION.join("unskilled_words");
    let _ = fs::create_dir_all(&directory);
    let mut result = vec![];
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let file_type = match entry.file_type() {
            Ok(x) => x,
            Err(_) => continue,
        };
        if file_type.is_dir() {
            continue;
        }
        let path = entry.path();
        let folder_name = match path.file_name() {
            None => continue,
            Some(x) => x,
        }
        .to_string_lossy()
        .to_string()
        .parse::<u128>();
        match folder_name {
            Ok(x) => result.push(x),
            Err(_) => continue,
        }
    }
    result
}

#[tauri::command]
fn load_unskilled_words(time: String) -> Vec<Word> {
    let path = DATA_LOCATION.join("unskilled_words").join(time);
    let _ = fs::create_dir_all(path.parent().unwrap());
    let raw_json = fs::read_to_string(path).unwrap();
    serde_json::from_str(&raw_json).unwrap()
}

#[tauri::command]
fn delete_unskilled_words_group(group: String) {
    let path = DATA_LOCATION.join("unskilled_words").join(group);
    let _ = fs::remove_file(path);
}
