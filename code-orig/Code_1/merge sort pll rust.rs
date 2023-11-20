use std::thread;
use std::time::{Instant};
use std::sync::{Arc, Mutex};

// number of elements in array
const MAX: usize = 10_000;
// number of threads
const THREAD_MAX: usize = 4;

// array of size MAX
static mut A: [i32; MAX] = [0; MAX];
static PART: Mutex<usize> = Mutex::new(0);

// merge function for merging two parts
fn merge(low: usize, mid: usize, high: usize) {
    let mut left = Vec::with_capacity(mid - low + 1);
    let mut right = Vec::with_capacity(high - mid);

    for i in low..=mid {
        left.push(unsafe { A[i] });
    }

    for i in mid + 1..=high {
        right.push(unsafe { A[i] });
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = low;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            unsafe {
                A[k] = left[i];
            }
            i += 1;
        } else {
            unsafe {
                A[k] = right[j];
            }
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        unsafe {
            A[k] = left[i];
        }
        i += 1;
        k += 1;
    }

    while j < right.len() {
        unsafe {
            A[k] = right[j];
        }
        j += 1;
        k += 1;
    }
}

// merge sort function
fn merge_sort(low: usize, high: usize) {
    if low < high {
        let mid = low + (high - low) / 2;
        let left_low = low;
        let left_high = mid;
        let right_low = mid + 1;
        let right_high = high;

        merge_sort(left_low, left_high);
        merge_sort(right_low, right_high);

        merge(low, mid, high);
    }
}

// thread function for multi-threading
fn merge_sort_thread() {
    let mut thread_part = PART.lock().unwrap();
    let thread_id = *thread_part;
    *thread_part += 1;

    let low = thread_id * (MAX / THREAD_MAX);
    let high = (thread_id + 1) * (MAX / THREAD_MAX) - 1;

    let mid = low + (high - low) / 2;
    merge_sort(low, mid);
    merge_sort(mid + 1, high);
    drop(thread_part);
}

fn main() {
    // Generating random values in array
    for i in 0..MAX {
        unsafe {
            A[i] = rand::random::<i32>() % 100;
        }
    }

    // Timing the merge sort algorithm
    let start_time = Instant::now();

    let mut threads = vec![];

    for _ in 0..THREAD_MAX {
        let handle = thread::spawn(|| merge_sort_thread());
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    merge(0, (MAX / 2 - 1) / 2, MAX / 2 - 1);
    merge(MAX / 2, MAX / 2 + (MAX - 1 - MAX / 2) / 2, MAX - 1);
    merge(0, (MAX - 1) / 2, MAX - 1);

    let end_time = Instant::now();

    // Displaying sorted array (commented out to avoid excessive output)
    // println!("Sorted array: {:?}", unsafe { &A });

    // Calculating and printing the time taken in seconds
    let elapsed_time = end_time - start_time;
    println!("Time taken: Parallel: {} seconds", elapsed_time.as_secs_f64());
    unsafe {println!("Array: {:?}", A);}
}
