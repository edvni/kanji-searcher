use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct KanjiEntry {
    #[serde(rename = "Kanji")]
    kanji: String,
    #[serde(rename = "Strokes")]
    strokes: u32,
    #[serde(rename = "Translation of Kun")]
    kun: String,
    #[serde(rename = "Translation of On")]
    on: String,
}

#[wasm_bindgen]
pub fn search_kanji_by_strokes(kanji_data: &str, stroke_count: u32) -> JsValue {
    let data: Vec<KanjiEntry> = serde_json::from_str(kanji_data).unwrap();
    
    let filtered_kanjis: Vec<KanjiEntry> = data.into_iter()
        .filter(|k| k.strokes == stroke_count)
        .collect();
    JsValue::from_serde(&filtered_kanjis).unwrap()
}