use rand::Rng;
use std::time::{Instant};

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

fn main()
{
	let end = 10000;
	let col0 = gen(end);

	let timer = Instant::now();
  let mut col1 = Vec::new();
  for (r, _) in col0.iter().enumerate()
	{
		col1.push(sum(&col0, r as u32, end));
	}
	let sum = sum(&col1, 0, end);

	println!("rows {}", end);
	println!("sum {}", sum);
	println!("{} ms", timer.elapsed().as_millis());
}
