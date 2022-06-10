use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Counter {
    key: char,
    count: i32,
}

#[wasm_bindgen]
impl Counter {
    pub fn default() -> Self {
        log("Counter::default");
        Self::new('a', 0)
    }

    pub fn new(key: char, count: i32) -> Counter {
        log(&format!("Counter::new({}, {}", key, count));
        Counter { key, count }
    }

    pub fn key(&self) -> char {
        log("Counter.key()");
        self.key
    }

    pub fn counter(&self) -> i32 {
        log("Counter.count");
        self.count
    }

    pub fn increment(&mut self) {
        log("Counter increment");
        self.count += 1;
    }

    pub fn update_key(&mut self, key: char) {
        self.key = key;
    }
}