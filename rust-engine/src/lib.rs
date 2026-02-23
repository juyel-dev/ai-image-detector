use wasm_bindgen::prelude::*;
use image::{DynamicImage, ImageBuffer, Luma};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectionResult {
    pub ai_probability: f32,
    pub confidence: f32,
    pub breakdown: HashMap<String, f32>,
    pub heatmap_base64: Option<String>,
    pub message: Option<String>,
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn analyze_image(image_bytes: Vec<u8>) -> JsValue {
    let result = match perform_detection(&image_bytes) {
        Ok(r) => r,
        Err(e) => DetectionResult {
            ai_probability: 0.5,
            confidence: 0.0,
            breakdown: HashMap::new(),
            heatmap_base64: None,
            message: Some(e),
        },
    };

    serde_wasm_bindgen::to_value(&result).unwrap()
}

fn perform_detection(bytes: &[u8]) -> Result<DetectionResult, String> {
    let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
    let gray = img.to_luma8();

    let entropy = calculate_entropy(&gray);
    let entropy_score = (entropy / 7.5).clamp(0.0, 1.0);

    let variance = calculate_variance(&gray);
    let sharpness_score = (variance / 800.0).clamp(0.0, 1.0);

    let ai_prob =
        (0.6 * (1.0 - entropy_score) + 0.4 * (1.0 - sharpness_score))
        .clamp(0.0, 1.0);

    let mut breakdown = HashMap::new();
    breakdown.insert("entropy".into(), entropy);
    breakdown.insert("variance".into(), variance);

    Ok(DetectionResult {
        ai_probability: ai_prob,
        confidence: 0.75,
        breakdown,
        heatmap_base64: None,
        message: None,
    })
}

fn calculate_entropy(gray: &ImageBuffer<Luma<u8>, Vec<u8>>) -> f32 {
    let mut hist = [0u32; 256];
    for p in gray.pixels() {
        hist[p[0] as usize] += 1;
    }

    let total = (gray.width() * gray.height()) as f32;
    let mut ent = 0.0;

    for &c in hist.iter() {
        if c > 0 {
            let p = c as f32 / total;
            ent -= p * p.ln();
        }
    }
    ent
}

fn calculate_variance(gray: &ImageBuffer<Luma<u8>, Vec<u8>>) -> f32 {
    let pixels: Vec<f32> = gray.pixels().map(|p| p[0] as f32).collect();
    let mean = pixels.iter().sum::<f32>() / pixels.len() as f32;

    pixels.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f32>() / pixels.len() as f32
      }
