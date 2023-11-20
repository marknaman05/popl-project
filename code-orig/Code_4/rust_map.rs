use std::time::{Instant};

use std::collections::HashMap;

const Max: usize = 1024;

fn main() {
    let mut scores: HashMap<i32, i32> = HashMap::new();

    for i in 0..Max {
        scores.insert(i as i32, i as i32);
    }

    let mut sum = 0;

    let start_time = Instant::now();
    
    // Iterate through the HashMap and calculate the sum of values
    for &value in scores.values() {
        sum += value;
    }
    
    let end_time = Instant::now();
    

    println!("The sum of values in the HashMap is: {}", sum);
    let elapsed_time = end_time - start_time;
    println!("Time taken: {} seconds", elapsed_time.as_secs_f64());
}