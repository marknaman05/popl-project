use std::time::{Instant};

fn main() {
    let start_time = Instant::now();
    let n = 10_000;
    
    let mut v: Vec<f64> = vec![0.0; n];

    for i in 0..n {
        v[i] = (i as f64).sqrt();
    }

    // Uncomment the following lines if you want to print the vector elements
    // for i in 0..n {
    //     println!("{}", v[i]);
    // }

    let end_time = Instant::now();
    
    let duration = end_time - start_time;
    println!("Time required to run in serial: {}ms", duration.as_millis());
}
