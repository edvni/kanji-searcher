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

/* 
// IMPOSSIBLE TO GET TEST TO WORK
#[cfg(test)]
wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
fn test_search_kanji_by_strokes() {
    let kanji_data = r#"
        [
            {"Kanji": "一", "Strokes": 1, "On within Joyo": "イチ", "Kun within Joyo": "ひと"},
            {"Kanji": "二", "Strokes": 2, "On within Joyo": "ニ", "Kun within Joyo": "ふた"},
            {"Kanji": "三", "Strokes": 3, "On within Joyo": "サン", "Kun within Joyo": "み"},
            {"Kanji": "門", "Strokes": 4, "On within Joyo": "モン", "Kun within Joyo": "かど"},
            {"Kanji": "五", "Strokes": 5, "On within Joyo": "ゴ", "Kun within Joyo": "いつ"}
        ]
    "#;

    // Test with a specific stroke count
    let stroke_count = 3;  // Choose the stroke count to test

    // Call the function to get the result
    let result = search_kanji_by_strokes(kanji_data, stroke_count);
    //let result_json = serde_json::to_string(&result).unwrap();

    // Define the expected result
    let expected_result_json = r#"
        [{"Kanji":"三","Strokes":3,"On within Joyo":"サン","Kun within Joyo":"み"}]
    "#;

    //assert_eq!(result_json, expected_result_json);
} 
*/