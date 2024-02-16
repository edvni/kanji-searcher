#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;
use wasm_bindgen::JsValue;
use serde_json::json;
use kanji_searcher::{search_kanji_by_strokes, KanjiEntry};

wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
fn test_search_kanji_by_strokes() {
    let data_json = json!(
    [
        {
            "Kanji": "憶",
            "Strokes": 16,
            "Translation of Kun": "-",
            "Translation of On": "think, remember"
          },
          {
            "Kanji": "臆",
            "Strokes": 17,
            "Translation of Kun": "-",
            "Translation of On": "timidity; breast, heart, mind"
          },
          {
            "Kanji": "虞",
            "Strokes": 13,
            "Translation of Kun": "fear; anxiety; concern; uneasiness",
            "Translation of On": "-"
          },
          {
            "Kanji": "一",
            "Strokes": 1,
            "Translation of Kun": "one, a unit; the same; just; once",
            "Translation of On": "one, a"
          },
          {
            "Kanji": "乙",
            "Strokes": 1,
            "Translation of Kun": "-",
            "Translation of On": "B, second; the latter; duplicate"
          },    {
            "Kanji": "遺",
            "Strokes": 15,
            "Translation of Kun": "-",
            "Translation of On": "leave behind; bequeath; save, reserve"
          },
          {
            "Kanji": "緯",
            "Strokes": 16,
            "Translation of Kun": "-",
            "Translation of On": "woof; horizontal; left and right; parallels of latitude; latitude"
          },
          {
            "Kanji": "域",
            "Strokes": 11,
            "Translation of Kun": "-",
            "Translation of On": "region; limits; stage, level"
          },
          {
            "Kanji": "育",
            "Strokes": 8,
            "Translation of Kun": "be raised, be brought up, grow, grow up; raise, rear, bring up",
            "Translation of On": "be raised, be brought up, grow, grow up"
          }
    ]).to_string();

    let testData_json = json!(
    [
        {
            "Kanji": "一",
            "Strokes": 1,
            "Translation of Kun": "one, a unit; the same; just; once",
            "Translation of On": "one, a"
          },
          {
            "Kanji": "乙",
            "Strokes": 1,
            "Translation of Kun": "-",
            "Translation of On": "B, second; the latter; duplicate"
          }
    ]);

    let testData: Vec<KanjiEntry> = serde_json::from_value(testData_json).unwrap();
    let expected = JsValue::from_serde(&testData).unwrap();
    let expected_string = expected.as_string();
    let data: &str = &data_json;
    let result = search_kanji_by_strokes(data, 1);
    let result_string = result.as_string();
    assert_eq!(result_string, expected_string);
}