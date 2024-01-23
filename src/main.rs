/*
 * Find all primes between 1 and 10^8 using 8 threads and a mutex to synchronize
 * access to a counter.
 * This is a very inefficient way to find primes, but it demonstrates the use of
 * dynamic load balancing and mutexes.
 * 
 * Compile with: rustc -O -o primes src/main.rs
 * Run with:     ./primes
 *
 * Write all primes to a file
 */ 

use std::sync::{Arc, Mutex};
use std::thread;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i + 6;
    }
    return true;
}

fn main() {
    // Define the number of threads and the number of primes to find.
    let nthreads = 8;
    let n = 100000000;

    // Start the timer.
    let start = std::time::Instant::now();

    // Create a vector to hold the thread handles.
    let mut handles = vec![];

    // Create a mutex-protected counter to keep track of the number of primes
    let counter = Arc::new(Mutex::new(0));
    let num_primes = Arc::new(Mutex::new(0));
    let sum_primes = Arc::new(Mutex::new(0));

    // Spawn nthreads threads.
    for _ in 0..nthreads {
        // Clone the mutex-protected counter for use in the thread.
        let counter = counter.clone();
        let num_primes = num_primes.clone();
        let sum_primes = sum_primes.clone();

        // Spawn a thread to find primes.
        let handle = thread::spawn(move || {
            loop {
                // Lock the mutex and increment the counter.
                let mut i = counter.lock().unwrap();
                if *i >= n {
                    break;
                }
                *i += 1;

                // Check if the counter value is prime.
                if is_prime(*i) {
                    let mut num = num_primes.lock().unwrap();
                    *num += 1;
                    let mut sum = sum_primes.lock().unwrap();
                    *sum += *i;
                    print!("{} ", *i)
                }
            }
        });

        // Add the thread handle to the vector.
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // Stop the timer.
    let duration = start.elapsed();

    // Print the number of primes found.
    let num = num_primes.lock().unwrap();
    let sum = sum_primes.lock().unwrap();
    println!("{} {} {}", duration.as_secs_f64(), *num, *sum);
}