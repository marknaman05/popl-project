use std::time::{Instant};
use rand::Rng;

fn merge_sort(mut vec: Vec<i32>) -> Vec<i32> {
    let len = vec.len();
    if len <= 1 {
        return vec; // If the input is empty or has only one element, it's already sorted.
    }

    let mid = len / 2;
    let left = merge_sort(vec[..mid].to_vec()); // Recursively sort the left half.
    let right = merge_sort(vec[mid..].to_vec()); // Recursively sort the right half.

    // Merge the sorted left and right halves.
    let mut merged = Vec::with_capacity(len);
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    // Append any remaining elements from the left and right subarrays.
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);

    merged
}

fn main() {
    const MAX: usize = 10_000;
    let mut A: Vec<i32> = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..MAX {
        A.push(rng.gen_range(0..100)); // Generate random integers in the range [0, 99].
    }

    let start_time = Instant::now();
    let sorted_array = merge_sort(A.clone()); // Clone the vector to keep the original intact.
    let end_time = Instant::now();

    println!("Sorted Array: {:?}", sorted_array);
    let elapsed_time = end_time - start_time;
    println!("Time taken: {} seconds", elapsed_time.as_secs_f64());
}
