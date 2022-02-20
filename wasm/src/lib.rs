mod utils;

use wasm_bindgen::prelude::*;
extern crate web_sys;
use rand::Rng;

fn gen(end: u32) -> Vec<f32>
{
	let mut rng = rand::thread_rng();
	let mut arr = Vec::new();
	for _ in 0..end
	{
		arr.push(rng.gen_range(0.0..1.0));
	}
	return arr;
}

fn sum(arr: &Vec<f32>, row: u32, end: u32) -> f32
{
	let mut sum = 0.0;
	for r in row..end
	{
		sum += arr[r as usize];
	}
	return sum;
}

fn calc() -> (u32, f32, f64)
{
	let end = 10000;
	let col0 = gen(end);

	let window = web_sys::window().expect("should have a window in this context");
	let performance = window
			.performance()
			.expect("performance should be available");

	let t_start = performance.now();
	let mut col1 = Vec::new();
	for (r, _) in col0.iter().enumerate()
	{
		col1.push(sum(&col0, r as u32, end));
	}
	let sum = sum(&col1, 0, end);
	let t_end = performance.now();

	return (end, sum, t_end - t_start);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	let res = calc();
	if !name.is_empty() {
		log(&format!("Hello, {}", name));
	}
	log(&format!("rows {}", res.0));
	log(&format!("sum {}", res.1));
	log(&format!("{} ms", res.2));
}
