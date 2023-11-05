use std::time::{Instant, Duration};
use rand::Rng;
use rayon::prelude::*;

const TEST_SIZE: usize = 1_000_000;
const ITERATION_COUNT: usize = 5;

fn print_results(tag: &str, sorted: &Vec<f64>, start_time: Instant, end_time: Instant) {
    let duration = end_time - start_time;
    println!(
        "{}: Lowest: {} Highest: {} Time: {:.2}ms",
        tag,
        sorted[0],
        sorted[TEST_SIZE - 1],
        duration.as_secs_f64() * 1000.0
    );
}

fn main() {
    // Generate some random doubles:
    println!("Testing with {} doubles...", TEST_SIZE);
    let mut rng = rand::thread_rng();
    let mut doubles: Vec<f64> = Vec::with_capacity(TEST_SIZE);

    for _ in 0..TEST_SIZE {
        doubles.push(rng.gen::<f64>()); // generating 10^6 random numbers
    }

    // Time how long it takes to sort them in parallel using Rayon:
    for _ in 0..ITERATION_COUNT {
        let mut sorted = doubles.clone();
        let start_time = Instant::now();
        
        sorted.par_sort_unstable(); // Parallel sorting using Rayon
        
        let end_time = Instant::now();
        print_results("Parallel", &sorted, start_time, end_time);
    }
}
