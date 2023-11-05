use std::time::{Instant};
use rayon::prelude::*;

fn main() {
    let start_time = Instant::now();
    let n = 10_000;

    let mut v: Vec<f64> = vec![0.0; n];

    v.par_iter_mut().enumerate().for_each(|(i, val)| {
        *val = (i as f64).sqrt();
    });

    // Uncomment the following lines if you want to print the vector elements
    // for i in 0..n {
    //     println!("{}", v[i]);
    // }

    let end_time = Instant::now();

    let duration = end_time - start_time;
    println!("Time required to run in parallel: {}ms", duration.as_millis());
}
