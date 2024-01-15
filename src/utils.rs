#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use once_cell::sync::Lazy;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(not(target_arch = "wasm32"))]
static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn getDevicePixelRatio() -> f64;
}

#[cfg(target_arch = "wasm32")]
pub fn device_pixel_ratio() -> f64 {
    getDevicePixelRatio()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn getCurrentTime() -> f32;
}

#[cfg(target_arch = "wasm32")]
pub fn get_time() -> f32 {
    getCurrentTime() / 1e3f32
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_time() -> f32 {
    // Fallback for non-WASM targets, using SystemTime or another method
    START_TIME.elapsed().as_secs_f64() as f32
}