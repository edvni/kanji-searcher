use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize};
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct KanjiEntry {
    #[serde(rename = "Kanji")]
    kanji: String,
    #[serde(rename = "Strokes")]
    strokes: u32,
    #[serde(rename = "On within Joyo")]
    on_within_joyo: String,
    #[serde(rename = "Kun within Joyo")]
    kun_within_joyo: String,
}

#[wasm_bindgen]
pub fn search_kanji_by_strokes(kanji_data: &str, stroke_count: u32) -> JsValue {
    let kanjis: Vec<KanjiEntry> = serde_json::from_str(kanji_data).unwrap_or_else(|_| vec![]);
    
    log(&format!("Total kanjis {}", kanjis.len()));
    
    let filtered_kanjis: Vec<KanjiEntry> = kanjis.into_iter()
        .filter(|k| k.strokes == stroke_count)
        .collect();
    JsValue::from_serde(&filtered_kanjis).unwrap()
}

#[cfg(test)]
#[wasm_bindgen_test]
fn test_search_kanji_by_strokes() {
    let kanji_data = r#"
    [
        {"Kanji": "一", "Strokes": 1, "On_within_Joyo": "ichi, itsu,", "Kun_within_Joyo": "hito, hito(tsu)"},
        {"Kanji": "二", "Strokes": 2, "On_within_Joyo": "ni,", "Kun_within_Joyo": "futa, futa(tsu)"},
        {"Kanji": "三", "Strokes": 3, "On_within_Joyo": "san", "Kun_within_Joyo": "mi, mi(tsu), mit(tsu)"}
    ]
    "#;

    let result = search_kanji_by_strokes(kanji_data, 2);
    let expected_result = r#"[{"Kanji":"二","Strokes":2,"On_within_Joyo":"","Kun_within_Joyo":""}]"#;
    assert_eq!(result, expected_result);
}
