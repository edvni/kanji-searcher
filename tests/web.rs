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
      "Kanji": "春",
      "Strokes": 9,
      "Translation of Kun": "spring, springtime",
      "Translation of On": "spring"
      },
      {
        "Kanji": "瞬",
        "Strokes": 18,
        "Translation of Kun": "wink, twinkle, flicker",
        "Translation of On": "wink, blink"
      },
      {
        "Kanji": "旬",
        "Strokes": 6,
        "Translation of Kun": "-",
        "Translation of On": "ten-day period. season (for specific products)"
      },
      {
        "Kanji": "巡",
        "Strokes": 6,
        "Translation of Kun": "go around; in connection with, concering",
        "Translation of On": "going around; circumference"
      },
      {
        "Kanji": "盾",
        "Strokes": 9,
        "Translation of Kun": "shield, buckler, escutcheon; pretext",
        "Translation of On": "shield, buckler, escutcheon; pretext"
      },
      {
        "Kanji": "准",
        "Strokes": 10,
        "Translation of Kun": "-",
        "Translation of On": "quasi-, semi-, associate"
      },
      {
        "Kanji": "殉",
        "Strokes": 10,
        "Translation of Kun": "-",
        "Translation of On": "die a martyr, follow (someone) by committing suicide; follow in resigning"
      },
      {
        "Kanji": "純",
        "Strokes": 10,
        "Translation of Kun": "-",
        "Translation of On": "purity, innocence; net (profit)"
      },
      {
        "Kanji": "循",
        "Strokes": 12,
        "Translation of Kun": "-",
        "Translation of On": "follow"
      },
    ]).to_string();

    let testData_json = json!(
    [
      {
        "Kanji": "純",
        "Strokes": 10,
        "Translation of Kun": "-",
        "Translation of On": "purity, innocence; net (profit)"
      },
      {
        "Kanji": "循",
        "Strokes": 12,
        "Translation of Kun": "-",
        "Translation of On": "follow"
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